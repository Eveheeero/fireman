use crate::{
    abstract_syntax_tree::{
        ArcAstVariableMap, Ast, AstExpression, AstFunctionId, AstFunctionVersion, AstLiteral,
        AstStatement, AstVariableId, ProcessedOptimization, Wrapped, WrappedAstStatement,
    },
    prelude::DecompileError,
};
use hashbrown::{HashMap, HashSet};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

/// Cached expression entry: stores everything needed to replace a later
/// duplicate with a variable reference.
struct CachedExpr {
    var_id: AstVariableId,
    /// The original RHS expression, kept for structural comparison (hash
    /// collisions are possible with `DefaultHasher`).
    expr: AstExpression,
    /// A pre-built `Variable(...)` wrapped expression that we clone into the
    /// replacement site.
    replacement: Wrapped<AstExpression>,
}

pub(super) fn eliminate_common_subexpressions(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
) -> Result<(), DecompileError> {
    let mut body;
    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        body = std::mem::take(&mut function.body);
    }

    let mut env: HashMap<u64, CachedExpr> = HashMap::new();
    cse_statement_list(&mut body, &mut env);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::CommonSubexpressionElimination);
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Statement list / statement processing
// ---------------------------------------------------------------------------

fn cse_statement_list(stmts: &mut Vec<WrappedAstStatement>, env: &mut HashMap<u64, CachedExpr>) {
    for stmt in stmts.iter_mut() {
        cse_statement(stmt, env);
    }
}

fn cse_statement(stmt: &mut WrappedAstStatement, env: &mut HashMap<u64, CachedExpr>) {
    match &mut stmt.statement {
        AstStatement::Assignment(lhs, rhs) => {
            // Try to replace RHS with a cached variable if it is a pure
            // duplicate, then record the new mapping.
            if let AstExpression::Variable(_, dst_id) = &lhs.item {
                let dst_id = *dst_id;
                if super::opt_utils::is_pure_expression(&rhs.item) {
                    let h = hash_expression(&rhs.item);
                    if let Some(cached) = env.get(&h) {
                        if super::opt_utils::expr_structurally_equal(&rhs.item, &cached.expr) {
                            // Replace the whole RHS with a clone of the cached
                            // variable expression.
                            *rhs = cached.replacement.clone();
                            // The destination is now an alias, but we do NOT
                            // insert it into env — its RHS is now just a
                            // variable, not the original expression.
                            // Invalidate any env entries whose var_id matches
                            // dst_id since it is being overwritten.
                            invalidate_var(env, dst_id);
                            return;
                        }
                    }
                    // No match — record this expression.
                    invalidate_var(env, dst_id);
                    if let AstExpression::Variable(vars, _) = &lhs.item {
                        let replacement = Wrapped {
                            item: AstExpression::Variable(vars.clone(), dst_id),
                            origin: rhs.origin.clone(),
                            comment: None,
                        };
                        env.insert(
                            h,
                            CachedExpr {
                                var_id: dst_id,
                                expr: rhs.item.clone(),
                                replacement,
                            },
                        );
                    }
                } else {
                    // Non-pure RHS: invalidate the overwritten variable.
                    invalidate_var(env, dst_id);
                }
            }
            // For non-variable LHS (e.g. deref) we don't track anything but
            // we don't need to clear env either — only calls/gotos/labels are
            // barriers.
        }
        AstStatement::Declaration(var, rhs) => {
            let var_id = var.id;
            if let Some(rhs) = rhs {
                if super::opt_utils::is_pure_expression(&rhs.item) {
                    let h = hash_expression(&rhs.item);
                    if let Some(cached) = env.get(&h) {
                        if super::opt_utils::expr_structurally_equal(&rhs.item, &cached.expr) {
                            *rhs = cached.replacement.clone();
                            invalidate_var(env, var_id);
                            return;
                        }
                    }
                    // Record — we need a variable map Arc for the replacement.
                    // Extract it from the RHS if it contains a variable, or
                    // from any cached entry, but the simplest correct approach
                    // is to look at the RHS: if it has at least one variable
                    // reference we can borrow its Arc.  For declarations the
                    // variable map should be the same function-level Arc used
                    // everywhere, so grab it from the first variable we find.
                    invalidate_var(env, var_id);
                    if let Some(vars) = extract_variable_map(&rhs.item) {
                        let replacement = Wrapped {
                            item: AstExpression::Variable(vars, var_id),
                            origin: rhs.origin.clone(),
                            comment: None,
                        };
                        env.insert(
                            h,
                            CachedExpr {
                                var_id,
                                expr: rhs.item.clone(),
                                replacement,
                            },
                        );
                    }
                    // If we cannot find a variable map (e.g. pure literal
                    // expression with no variable refs) we simply skip caching.
                    // Literal-only expressions are typically handled by constant
                    // folding instead.
                } else {
                    invalidate_var(env, var_id);
                }
            } else {
                invalidate_var(env, var_id);
            }
        }
        AstStatement::If(cond, branch_true, branch_false) => {
            // Condition is evaluated in the current env — but we do not try to
            // CSE the condition expression itself (it is not assigned to a
            // variable here).
            let _ = cond;
            let env_before = env.clone();
            let mut env_true = env_before.clone();
            cse_statement_list(branch_true, &mut env_true);
            if let Some(branch_false) = branch_false {
                let mut env_false = env_before;
                cse_statement_list(branch_false, &mut env_false);
                *env = intersect_envs(&env_true, &env_false);
            } else {
                *env = intersect_envs(&env_before, &env_true);
            }
        }
        AstStatement::While(cond, body) => {
            let _ = cond;
            // Invalidate any variables written inside the loop.
            let mut written = HashSet::new();
            collect_written_vars_list(body, &mut written);
            invalidate_written(env, &written);
            let mut env_body = env.clone();
            cse_statement_list(body, &mut env_body);
            // After loop: invalidate written vars again.
            invalidate_written(env, &written);
        }
        AstStatement::For(init, cond, update, body) => {
            cse_statement(init, env);
            let _ = cond;
            let mut written = HashSet::new();
            collect_written_vars_list(body, &mut written);
            collect_written_vars_stmt(&update.statement, &mut written);
            invalidate_written(env, &written);
            let mut env_body = env.clone();
            cse_statement_list(body, &mut env_body);
            cse_statement(&mut *update, &mut env_body);
            invalidate_written(env, &written);
        }
        AstStatement::Switch(discrim, cases, default) => {
            let _ = discrim;
            let env_before = env.clone();
            let mut branch_envs: Vec<HashMap<u64, CachedExpr>> = Vec::new();
            for (_lit, case_body) in cases.iter_mut() {
                let mut env_case = env_before.clone();
                cse_statement_list(case_body, &mut env_case);
                branch_envs.push(env_case);
            }
            if let Some(default_body) = default {
                let mut env_default = env_before.clone();
                cse_statement_list(default_body, &mut env_default);
                branch_envs.push(env_default);
            }
            if branch_envs.is_empty() {
                *env = env_before;
            } else {
                let mut result = branch_envs.remove(0);
                for other in &branch_envs {
                    result = intersect_envs(&result, other);
                }
                *env = result;
            }
        }
        AstStatement::Block(body) => {
            cse_statement_list(body, env);
        }
        AstStatement::Return(expr) => {
            let _ = expr;
            env.clear();
        }
        AstStatement::Call(_) => {
            // Calls may modify anything — full barrier.
            env.clear();
        }
        AstStatement::Goto(_)
        | AstStatement::Assembly(_)
        | AstStatement::Ir(_)
        | AstStatement::Undefined
        | AstStatement::Exception(_) => {
            env.clear();
        }
        AstStatement::Label(_) => {
            // Join point: cannot know which path reached here.
            env.clear();
        }
        AstStatement::Comment(_) | AstStatement::Empty => {}
    }
}

// ---------------------------------------------------------------------------
// Env helpers
// ---------------------------------------------------------------------------

/// Remove any env entries whose cached variable is `var_id` (it is being
/// overwritten, so the cached expression is no longer valid).
fn invalidate_var(env: &mut HashMap<u64, CachedExpr>, var_id: AstVariableId) {
    env.retain(|_, cached| {
        // Remove if the cached variable IS the one being overwritten.
        if cached.var_id == var_id {
            return false;
        }
        // Also remove if the cached expression *reads* this variable, because
        // the expression value may change when the variable is overwritten.
        let mut vars = hashbrown::HashSet::new();
        super::opt_utils::collect_expr_variables(&cached.expr, &mut vars);
        !vars.contains(&var_id)
    });
}

/// Invalidate env entries where either the cached var or any variable read by
/// the cached expression is in the written set.
fn invalidate_written(env: &mut HashMap<u64, CachedExpr>, written: &HashSet<AstVariableId>) {
    env.retain(|_, cached| {
        if written.contains(&cached.var_id) {
            return false;
        }
        let mut vars = hashbrown::HashSet::new();
        super::opt_utils::collect_expr_variables(&cached.expr, &mut vars);
        vars.iter().all(|v| !written.contains(v))
    });
}

/// Intersect two envs: keep only entries present in both with the same var_id
/// and structurally equal expressions.
fn intersect_envs(
    a: &HashMap<u64, CachedExpr>,
    b: &HashMap<u64, CachedExpr>,
) -> HashMap<u64, CachedExpr> {
    a.iter()
        .filter_map(|(hash, ca)| {
            b.get(hash).and_then(|cb| {
                if ca.var_id == cb.var_id
                    && super::opt_utils::expr_structurally_equal(&ca.expr, &cb.expr)
                {
                    Some((
                        *hash,
                        CachedExpr {
                            var_id: ca.var_id,
                            expr: ca.expr.clone(),
                            replacement: ca.replacement.clone(),
                        },
                    ))
                } else {
                    None
                }
            })
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Collect written variables (reused from other passes' pattern)
// ---------------------------------------------------------------------------

fn collect_written_vars_list(stmts: &[WrappedAstStatement], out: &mut HashSet<AstVariableId>) {
    for stmt in stmts {
        collect_written_vars_stmt(&stmt.statement, out);
    }
}

fn collect_written_vars_stmt(stmt: &AstStatement, out: &mut HashSet<AstVariableId>) {
    match stmt {
        AstStatement::Assignment(lhs, _) => {
            if let AstExpression::Variable(_, var_id) = &lhs.item {
                out.insert(*var_id);
            }
        }
        AstStatement::Declaration(var, _) => {
            out.insert(var.id);
        }
        AstStatement::If(_, bt, bf) => {
            collect_written_vars_list(bt, out);
            if let Some(bf) = bf {
                collect_written_vars_list(bf, out);
            }
        }
        AstStatement::While(_, body) => {
            collect_written_vars_list(body, out);
        }
        AstStatement::For(init, _, update, body) => {
            collect_written_vars_stmt(&init.statement, out);
            collect_written_vars_stmt(&update.statement, out);
            collect_written_vars_list(body, out);
        }
        AstStatement::Switch(_, cases, default) => {
            for (_lit, case_body) in cases {
                collect_written_vars_list(case_body, out);
            }
            if let Some(default_body) = default {
                collect_written_vars_list(default_body, out);
            }
        }
        AstStatement::Block(body) => {
            collect_written_vars_list(body, out);
        }
        _ => {}
    }
}

// ---------------------------------------------------------------------------
// Expression hashing
// ---------------------------------------------------------------------------

fn hash_expression(expr: &AstExpression) -> u64 {
    let mut hasher = DefaultHasher::new();
    hash_expr_recursive(expr, &mut hasher);
    hasher.finish()
}

fn hash_expr_recursive(expr: &AstExpression, hasher: &mut DefaultHasher) {
    // Discriminant tag first so different variants never collide.
    std::mem::discriminant(expr).hash(hasher);

    match expr {
        AstExpression::Unknown
        | AstExpression::Undefined
        | AstExpression::ArchitectureBitSize
        | AstExpression::ArchitectureByteSize => {}
        AstExpression::Literal(lit) => hash_literal(lit, hasher),
        AstExpression::Variable(_, var_id) => {
            var_id.hash(hasher);
        }
        AstExpression::UnaryOp(op, arg) => {
            std::mem::discriminant(op).hash(hasher);
            hash_expr_recursive(&arg.item, hasher);
        }
        AstExpression::BinaryOp(op, left, right) => {
            std::mem::discriminant(op).hash(hasher);
            hash_expr_recursive(&left.item, hasher);
            hash_expr_recursive(&right.item, hasher);
        }
        AstExpression::Cast(ty, arg) => {
            // AstValueType derives PartialEq; use debug repr for hashing.
            format!("{:?}", ty).hash(hasher);
            hash_expr_recursive(&arg.item, hasher);
        }
        AstExpression::Deref(arg) | AstExpression::AddressOf(arg) => {
            hash_expr_recursive(&arg.item, hasher);
        }
        AstExpression::ArrayAccess(base, idx) => {
            hash_expr_recursive(&base.item, hasher);
            hash_expr_recursive(&idx.item, hasher);
        }
        AstExpression::MemberAccess(arg, field) => {
            hash_expr_recursive(&arg.item, hasher);
            field.hash(hasher);
        }
        AstExpression::Ternary(cond, t, f) => {
            hash_expr_recursive(&cond.item, hasher);
            hash_expr_recursive(&t.item, hasher);
            hash_expr_recursive(&f.item, hasher);
        }
        AstExpression::Call(_) => {
            // Calls are not pure so they should never reach hashing, but
            // hash a constant sentinel to be safe.
            0xDEAD_BEEF_u64.hash(hasher);
        }
    }
}

fn hash_literal(lit: &AstLiteral, hasher: &mut DefaultHasher) {
    std::mem::discriminant(lit).hash(hasher);
    match lit {
        AstLiteral::Int(v) => v.hash(hasher),
        AstLiteral::UInt(v) => v.hash(hasher),
        AstLiteral::Float(v) => v.to_bits().hash(hasher),
        AstLiteral::String(v) => v.hash(hasher),
        AstLiteral::Char(v) => v.hash(hasher),
        AstLiteral::Bool(v) => v.hash(hasher),
    }
}

// ---------------------------------------------------------------------------
// Utilities
// ---------------------------------------------------------------------------

/// Walk an expression tree and return the first `ArcAstVariableMap` found.
/// All variables within a single function share the same Arc, so any one
/// will do.
fn extract_variable_map(expr: &AstExpression) -> Option<ArcAstVariableMap> {
    match expr {
        AstExpression::Variable(vars, _) => Some(vars.clone()),
        AstExpression::UnaryOp(_, arg)
        | AstExpression::Cast(_, arg)
        | AstExpression::Deref(arg)
        | AstExpression::AddressOf(arg)
        | AstExpression::MemberAccess(arg, _) => extract_variable_map(&arg.item),
        AstExpression::BinaryOp(_, left, right) | AstExpression::ArrayAccess(left, right) => {
            extract_variable_map(&left.item).or_else(|| extract_variable_map(&right.item))
        }
        AstExpression::Ternary(cond, t, f) => extract_variable_map(&cond.item)
            .or_else(|| extract_variable_map(&t.item))
            .or_else(|| extract_variable_map(&f.item)),
        AstExpression::Call(_)
        | AstExpression::Literal(_)
        | AstExpression::Unknown
        | AstExpression::Undefined
        | AstExpression::ArchitectureBitSize
        | AstExpression::ArchitectureByteSize => None,
    }
}

// ---------------------------------------------------------------------------
// Clone support for CachedExpr (needed for env snapshots at branches)
// ---------------------------------------------------------------------------

impl Clone for CachedExpr {
    fn clone(&self) -> Self {
        Self {
            var_id: self.var_id,
            expr: self.expr.clone(),
            replacement: self.replacement.clone(),
        }
    }
}
