//! Build and query the inter-procedural call graph.

use crate::abstract_syntax_tree::{
    Ast, AstCall, AstExpression, AstFunctionId, AstStatement, WrappedAstStatement,
};
use hashbrown::{HashMap, HashSet};

/// Minimal call graph built from AST function bodies.
pub struct CallGraph {
    /// For each function, the set of functions it directly calls.
    pub callees: HashMap<AstFunctionId, HashSet<AstFunctionId>>,
    /// For each function, the set of functions that call it.
    pub callers: HashMap<AstFunctionId, HashSet<AstFunctionId>>,
}

/// Build a call graph by walking all function bodies in the AST.
pub fn build_call_graph(ast: &Ast) -> CallGraph {
    let functions = ast.functions.read().unwrap();
    let mut callees: HashMap<AstFunctionId, HashSet<AstFunctionId>> = HashMap::new();
    let mut callers: HashMap<AstFunctionId, HashSet<AstFunctionId>> = HashMap::new();

    for (&function_id, version_map) in functions.iter() {
        let Some(&function_version) = ast.function_versions.get(&function_id) else {
            continue;
        };
        let Some(function) = version_map.get(&function_version) else {
            continue;
        };

        let mut targets = HashSet::new();
        collect_call_targets(&function.body, &mut targets);

        for &target in &targets {
            callers.entry(target).or_default().insert(function_id);
        }
        callees.insert(function_id, targets);
    }

    CallGraph { callees, callers }
}

fn collect_call_targets(stmts: &[WrappedAstStatement], out: &mut HashSet<AstFunctionId>) {
    for stmt in stmts {
        collect_call_targets_in_stmt(&stmt.statement, out);
    }
}

fn collect_call_targets_in_stmt(stmt: &AstStatement, out: &mut HashSet<AstFunctionId>) {
    match stmt {
        AstStatement::Call(call) => collect_call_target(call, out),
        AstStatement::Return(Some(expr)) => collect_call_targets_in_expr(&expr.item, out),
        AstStatement::If(cond, bt, bf) => {
            collect_call_targets_in_expr(&cond.item, out);
            collect_call_targets(bt, out);
            if let Some(bf) = bf {
                collect_call_targets(bf, out);
            }
        }
        AstStatement::While(cond, body) => {
            collect_call_targets_in_expr(&cond.item, out);
            collect_call_targets(body, out);
        }
        AstStatement::For(init, cond, update, body) => {
            collect_call_targets_in_stmt(&init.statement, out);
            collect_call_targets_in_expr(&cond.item, out);
            collect_call_targets_in_stmt(&update.statement, out);
            collect_call_targets(body, out);
        }
        AstStatement::Switch(disc, cases, default) => {
            collect_call_targets_in_expr(&disc.item, out);
            for (_, case_body) in cases {
                collect_call_targets(case_body, out);
            }
            if let Some(default_body) = default {
                collect_call_targets(default_body, out);
            }
        }
        AstStatement::Block(body) => collect_call_targets(body, out),
        AstStatement::Declaration(_, Some(init)) => {
            collect_call_targets_in_expr(&init.item, out);
        }
        AstStatement::Assignment(lhs, rhs) => {
            collect_call_targets_in_expr(&lhs.item, out);
            collect_call_targets_in_expr(&rhs.item, out);
        }
        _ => {}
    }
}

fn collect_call_targets_in_expr(expr: &AstExpression, out: &mut HashSet<AstFunctionId>) {
    match expr {
        AstExpression::Call(call) => collect_call_target(call, out),
        AstExpression::UnaryOp(_, arg) | AstExpression::Cast(_, arg) => {
            collect_call_targets_in_expr(&arg.item, out);
        }
        AstExpression::BinaryOp(_, l, r) => {
            collect_call_targets_in_expr(&l.item, out);
            collect_call_targets_in_expr(&r.item, out);
        }
        AstExpression::Deref(e)
        | AstExpression::AddressOf(e)
        | AstExpression::MemberAccess(e, _) => {
            collect_call_targets_in_expr(&e.item, out);
        }
        AstExpression::ArrayAccess(base, idx) => {
            collect_call_targets_in_expr(&base.item, out);
            collect_call_targets_in_expr(&idx.item, out);
        }
        AstExpression::Ternary(cond, t, f) => {
            collect_call_targets_in_expr(&cond.item, out);
            collect_call_targets_in_expr(&t.item, out);
            collect_call_targets_in_expr(&f.item, out);
        }
        _ => {}
    }
}

fn collect_call_target(call: &AstCall, out: &mut HashSet<AstFunctionId>) {
    if let AstCall::Function { target, args, .. } = call {
        out.insert(*target);
        for arg in args {
            collect_call_targets_in_expr(&arg.item, out);
        }
    }
}

/// Compute strongly-connected components of the call graph using Tarjan's algorithm.
/// Returns SCCs in reverse topological order (leaf SCCs first).
/// SCCs with more than one member represent mutually-recursive function clusters.
pub fn compute_sccs(graph: &CallGraph) -> Vec<Vec<AstFunctionId>> {
    struct TarjanState {
        index_counter: u32,
        stack: Vec<AstFunctionId>,
        on_stack: HashSet<AstFunctionId>,
        index: HashMap<AstFunctionId, u32>,
        lowlink: HashMap<AstFunctionId, u32>,
        result: Vec<Vec<AstFunctionId>>,
    }

    fn strongconnect(
        v: AstFunctionId,
        callees: &HashMap<AstFunctionId, HashSet<AstFunctionId>>,
        state: &mut TarjanState,
    ) {
        let idx = state.index_counter;
        state.index_counter += 1;
        state.index.insert(v, idx);
        state.lowlink.insert(v, idx);
        state.stack.push(v);
        state.on_stack.insert(v);

        if let Some(targets) = callees.get(&v) {
            for &w in targets {
                if !state.index.contains_key(&w) {
                    strongconnect(w, callees, state);
                    let w_low = state.lowlink[&w];
                    let v_low = state.lowlink.get_mut(&v).unwrap();
                    if w_low < *v_low {
                        *v_low = w_low;
                    }
                } else if state.on_stack.contains(&w) {
                    let w_idx = state.index[&w];
                    let v_low = state.lowlink.get_mut(&v).unwrap();
                    if w_idx < *v_low {
                        *v_low = w_idx;
                    }
                }
            }
        }

        if state.lowlink[&v] == state.index[&v] {
            let mut component = Vec::new();
            loop {
                let w = state.stack.pop().unwrap();
                state.on_stack.remove(&w);
                component.push(w);
                if w == v {
                    break;
                }
            }
            component.sort_unstable();
            state.result.push(component);
        }
    }

    let mut all_nodes: Vec<AstFunctionId> = graph.callees.keys().copied().collect();
    all_nodes.sort_unstable();

    let mut state = TarjanState {
        index_counter: 0,
        stack: Vec::new(),
        on_stack: HashSet::new(),
        index: HashMap::new(),
        lowlink: HashMap::new(),
        result: Vec::new(),
    };

    for &node in &all_nodes {
        if !state.index.contains_key(&node) {
            strongconnect(node, &graph.callees, &mut state);
        }
    }

    state.result
}

/// Rank functions by "importance": number of distinct callers (in-degree).
/// Returns a sorted list of (function_id, caller_count) pairs, most-called first.
pub fn rank_by_importance(graph: &CallGraph) -> Vec<(AstFunctionId, usize)> {
    let mut ranking: Vec<(AstFunctionId, usize)> = graph
        .callers
        .iter()
        .map(|(&fid, callers)| (fid, callers.len()))
        .collect();
    // Also include functions with no callers (entry points / dead code) with count 0.
    for &fid in graph.callees.keys() {
        if !graph.callers.contains_key(&fid) {
            ranking.push((fid, 0));
        }
    }
    ranking.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    ranking
}

/// Infer which functions are "pure" (no observable side effects in their body).
/// A function is pure if its body contains no calls with side effects and no
/// assignments to dereferenced pointers/arrays/globals.
///
/// Returns a set of function IDs deemed pure. Functions that only call other
/// pure functions (transitively) are also considered pure.
pub fn infer_pure_functions(ast: &Ast, graph: &CallGraph) -> HashSet<AstFunctionId> {
    let functions = ast.functions.read().unwrap();

    // First pass: classify functions whose bodies have no side-effect statements
    // (ignoring callees — we'll refine with call graph).
    let mut locally_pure: HashSet<AstFunctionId> = HashSet::new();
    for (&function_id, version_map) in functions.iter() {
        let Some(&function_version) = ast.function_versions.get(&function_id) else {
            continue;
        };
        let Some(function) = version_map.get(&function_version) else {
            continue;
        };
        if body_is_locally_pure(&function.body) {
            locally_pure.insert(function_id);
        }
    }

    // Iterative refinement: a locally-pure function is truly pure only if
    // all its callees are also pure.
    let mut pure = locally_pure.clone();
    loop {
        let mut changed = false;
        for &fid in &locally_pure {
            if !pure.contains(&fid) {
                continue;
            }
            if let Some(callees) = graph.callees.get(&fid) {
                for &callee in callees {
                    if !pure.contains(&callee) {
                        pure.remove(&fid);
                        changed = true;
                        break;
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }

    pure
}

/// Check if a function body is locally pure (no writes through pointers, no
/// impure expressions in statements). Calls are ignored — they're checked
/// transitively via the call graph.
fn body_is_locally_pure(stmts: &[WrappedAstStatement]) -> bool {
    for stmt in stmts {
        if !stmt_is_locally_pure(&stmt.statement) {
            return false;
        }
    }
    true
}

fn stmt_is_locally_pure(stmt: &AstStatement) -> bool {
    match stmt {
        // Assignments to dereferenced/array/member targets are side-effectful
        AstStatement::Assignment(lhs, _rhs) => {
            matches!(&lhs.item, AstExpression::Variable(_, _))
        }
        AstStatement::Declaration(_, _) => true,
        AstStatement::Return(_) => true,
        AstStatement::Empty | AstStatement::Comment(_) => true,
        AstStatement::If(_, bt, bf) => {
            body_is_locally_pure(bt) && bf.as_ref().map_or(true, |bf| body_is_locally_pure(bf))
        }
        AstStatement::While(_, body) | AstStatement::Block(body) => body_is_locally_pure(body),
        AstStatement::For(init, _, update, body) => {
            stmt_is_locally_pure(&init.statement)
                && stmt_is_locally_pure(&update.statement)
                && body_is_locally_pure(body)
        }
        AstStatement::Switch(_, cases, default) => {
            cases.iter().all(|(_, cb)| body_is_locally_pure(cb))
                && default.as_ref().map_or(true, |d| body_is_locally_pure(d))
        }
        // Calls are checked transitively, so local check considers them pure
        AstStatement::Call(_) => true,
        // Gotos, labels, assembly, etc. are impure
        _ => false,
    }
}

/// Returns the set of function IDs that participate in recursion
/// (i.e., belong to an SCC of size > 1, or self-recursive).
pub fn find_recursive_functions(graph: &CallGraph) -> HashSet<AstFunctionId> {
    let sccs = compute_sccs(graph);
    let mut recursive = HashSet::new();
    for scc in &sccs {
        if scc.len() > 1 {
            for &fid in scc {
                recursive.insert(fid);
            }
        } else if scc.len() == 1 {
            let fid = scc[0];
            if let Some(targets) = graph.callees.get(&fid) {
                if targets.contains(&fid) {
                    recursive.insert(fid);
                }
            }
        }
    }
    recursive
}
