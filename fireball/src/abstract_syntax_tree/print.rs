use super::*;
use hashbrown::{HashMap, HashSet};

fn format_descriptor(ir_index: u32, statement_index: Option<u8>) -> String {
    match statement_index {
        Some(stmt_index) => format!("{}:{}", ir_index, stmt_index),
        None => format!("{}", ir_index),
    }
}

fn format_parameter_location(location: &AstParameterLocation) -> String {
    match location {
        AstParameterLocation::Register(reg) => format!("reg {}", reg),
        AstParameterLocation::Stack(offset) => format!("stack {:+#x}", offset),
    }
}

fn variable_usage_summary(var: &AstVariable) -> String {
    let Some(access_map) = var.data_access_ir.as_ref() else {
        return "usage: n/a".to_string();
    };
    if access_map.is_empty() {
        return "usage: none".to_string();
    }

    let mut points: Vec<_> = access_map
        .iter()
        .map(|(descriptor, _)| {
            (
                descriptor.ir_index(),
                *descriptor.statement_index(),
                format_descriptor(descriptor.ir_index(), *descriptor.statement_index()),
            )
        })
        .collect();
    points.sort_unstable_by_key(|(ir_index, stmt_index, _)| (*ir_index, *stmt_index));

    let mut read_count = 0usize;
    let mut write_count = 0usize;
    let mut where_used: Vec<String> = access_map
        .iter()
        .flat_map(|(_, accesses)| accesses.iter())
        .map(|access| {
            match access.access_type() {
                crate::ir::data::IrDataAccessType::Read => read_count += 1,
                crate::ir::data::IrDataAccessType::Write => write_count += 1,
            }
            format!("{} {}", access.access_type(), access.location())
        })
        .collect();
    where_used.sort_unstable();
    where_used.dedup();
    let omitted = where_used.len().saturating_sub(4);
    where_used.truncate(4);
    let where_used = if where_used.is_empty() {
        "where n/a".to_string()
    } else if omitted > 0 {
        format!("where {} (+{} more)", where_used.join(", "), omitted)
    } else {
        format!("where {}", where_used.join(", "))
    };

    let range = if let (Some(first), Some(last)) = (points.first(), points.last()) {
        format!("{}..{}", first.2, last.2)
    } else {
        "n/a".to_string()
    };
    format!(
        "range {}, access r:{} w:{}, {}",
        range, read_count, write_count, where_used
    )
}

fn parameter_usage_comment(param: &AstParameter, variables: &ArcAstVariableMap) -> String {
    let location = format_parameter_location(&param.location);
    let usage = match &param.id {
        either::Either::Left(var_id) => {
            let vars = variables.read().unwrap();
            vars.get(var_id)
                .map(variable_usage_summary)
                .unwrap_or_else(|| "usage: unresolved variable".to_string())
        }
        either::Either::Right(_) => "usage: no linked variable".to_string(),
    };
    format!("param {} | {}", location, usage)
}

fn push_indented_lines(output: &mut String, indent: &str, content: &str) {
    for line in content.lines() {
        output.push_str(indent);
        output.push_str(line);
        output.push('\n');
    }
}

fn collect_statement_ir_origins<'a>(
    origin: &'a AstStatementOrigin,
    out: &mut Vec<&'a AstDescriptor>,
) {
    match origin {
        AstStatementOrigin::Ir(descriptor) => out.push(descriptor),
        AstStatementOrigin::Combination(origins) => {
            for origin in origins {
                collect_statement_ir_origins(origin, out);
            }
        }
        _ => {}
    }
}

fn descriptor_source_key(descriptor: &AstDescriptor) -> usize {
    std::sync::Arc::as_ptr(descriptor.ir()) as usize
}

impl Ast {
    pub fn print(&self, config: Option<AstPrintConfig>) -> String {
        let config = config.unwrap_or_default();
        let mut output = String::new();
        let function_versions = &self.function_versions;

        // Functions
        let functions = self.functions.read().unwrap();
        let mut function_keys_sorted = functions.keys().collect::<Vec<_>>();
        function_keys_sorted.sort_by_cached_key(|key_ref| {
            let key = *key_ref;
            let is_main = function_versions
                .get(key)
                .and_then(|version| functions.get(key).and_then(|m| m.get(version)))
                .is_some_and(|function| function.name() == "main");
            (if is_main { 0u8 } else { 1u8 }, key.address)
        });
        for func_id in function_keys_sorted {
            let version_map = functions.get(func_id).unwrap();
            let version = function_versions.get(func_id).unwrap();
            let func = version_map.get(version).unwrap();
            output.push_str(&format!(
                "{} {}(",
                func.return_type.to_string_with_config(Some(config)),
                func.name()
            ));

            // Parameters
            if !func.parameters.is_empty() {
                let params: Vec<String> = func
                    .parameters
                    .iter()
                    .map(|param| {
                        let ty_str = param
                            .read_type(&func.variables)
                            .expect("invalid variable map")
                            .to_string_with_config(Some(config));
                        let name_str = param.name(&func.variables).expect("invalid variable map");
                        let mut line = format!("{} {}", ty_str, name_str);
                        if config.parameter_usage_comment {
                            line.push_str(&format!(
                                " /* {} */",
                                parameter_usage_comment(param, &func.variables)
                            ));
                        }
                        line
                    })
                    .collect();
                output.push_str("\n  ");
                output.push_str(&params.join(",\n  "));
                output.push('\n');
            }

            output.push_str(") {\n");

            // Local variables
            {
                let body_vars: Option<HashSet<AstVariableId>> = if config.hide_unused_declarations {
                    let mut vars = HashSet::new();
                    for stmt in &func.body {
                        for (_, var_id) in stmt.statement.get_related_variables() {
                            vars.insert(var_id);
                        }
                    }
                    Some(vars)
                } else {
                    None
                };
                let var_map = func.variables.read().unwrap();
                let mut var_keys_sorted: Vec<_> = var_map
                    .keys()
                    .filter(|k| body_vars.as_ref().is_none_or(|bv| bv.contains(*k)))
                    .collect();
                var_keys_sorted.sort_by_cached_key(|key| {
                    let (kind_priority, parent_addr) = if key.parent == Some(func.id) {
                        (0u8, func.id.address)
                    } else if key.parent.is_none() {
                        (1u8, 0)
                    } else {
                        (2u8, key.parent.map(|id| id.address).unwrap_or(0))
                    };
                    (kind_priority, parent_addr, key.index)
                });
                let mut decl_rows: Vec<((u8, u64), String, String, Option<String>)> = Vec::new();
                for var_key in var_keys_sorted {
                    let var = var_map.get(var_key).unwrap();
                    let group_key = if var_key.parent == Some(func.id) {
                        (0u8, func.id.address)
                    } else if var_key.parent.is_none() {
                        (1u8, 0)
                    } else {
                        (2u8, var_key.parent.map(|id| id.address).unwrap_or(0))
                    };
                    if let Some(const_value) = &var.const_value {
                        if !config.replace_constant {
                            decl_rows.push((
                                group_key,
                                format!(
                                    "const {}",
                                    var.var_type.to_string_with_config(Some(config))
                                ),
                                format!(
                                    "{} = {}",
                                    var.name(),
                                    const_value.to_string_with_config(Some(config))
                                ),
                                config
                                    .variable_usage_comment
                                    .then(|| variable_usage_summary(var)),
                            ));
                        } else {
                            debug!(
                                function=?func.name(),
                                "{} {} was replaced with constant value {}",
                                var.var_type.to_string_with_config(Some(config)),
                                var.name(),
                                const_value.to_string_with_config(Some(config))
                            );
                        }
                    } else {
                        decl_rows.push((
                            group_key,
                            var.var_type.to_string_with_config(Some(config)),
                            var.name(),
                            config
                                .variable_usage_comment
                                .then(|| variable_usage_summary(var)),
                        ));
                    }
                }

                if !decl_rows.is_empty() {
                    let mut group_left_width: HashMap<(u8, u64), usize> = HashMap::new();
                    for (group_key, left, _, _) in decl_rows.iter() {
                        let width = group_left_width.entry(*group_key).or_insert(0);
                        *width = (*width).max(left.len());
                    }

                    let mut prev_group: Option<(u8, u64)> = None;
                    for (group_key, left, right, usage_comment) in decl_rows {
                        if prev_group.is_some() && prev_group != Some(group_key) {
                            output.push('\n');
                        }
                        let left_width = group_left_width.get(&group_key).copied().unwrap_or(0);
                        output.push_str(&format!(
                            "  {:<width$} {};",
                            left,
                            right,
                            width = left_width
                        ));
                        if let Some(comment) = usage_comment {
                            output.push_str(&format!(" /* {} */", comment));
                        }
                        output.push('\n');
                        prev_group = Some(group_key);
                    }
                    output.push_str("\n");
                }
            }

            // Function body
            let mut visited_ir = HashSet::new();
            let mut prev_stmt = None;
            for stmt in &func.body {
                let content = stmt.to_string_with_config(Some(config));
                if content.is_empty() {
                    continue;
                }
                let mut origins = Vec::new();
                collect_statement_ir_origins(&stmt.origin, &mut origins);
                if config.print_instruction {
                    for descriptor in &origins {
                        let source_key = descriptor_source_key(descriptor);
                        let ir_key = (source_key, descriptor.descriptor().ir_index());
                        if !visited_ir.contains(&ir_key) {
                            let instruction = &descriptor.ir().get_instructions()
                                [descriptor.descriptor().ir_index() as usize];
                            output.push_str(&format!("  // {}\n", instruction));
                            visited_ir.insert(ir_key);
                        }
                    }
                }
                if config.print_ir {
                    for descriptor in &origins {
                        if let Some(statement_index) = descriptor.descriptor().statement_index() {
                            let source_key = descriptor_source_key(descriptor);
                            let descriptor_key = (source_key, *descriptor.descriptor());
                            if prev_stmt != Some(descriptor_key) {
                                let stmt = &descriptor.ir().get_ir()
                                    [descriptor.descriptor().ir_index() as usize]
                                    .statements
                                    .as_ref()
                                    .unwrap()[*statement_index as usize];
                                output.push_str(&format!("    /* {} */\n", stmt));
                                prev_stmt = Some(descriptor_key);
                            }
                        }
                    }
                }
                push_indented_lines(&mut output, "    ", &content);
            }

            output.push_str("}\n\n");
        }

        output.pop();
        output
    }
}
