use super::*;
use hashbrown::HashSet;

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
        format!("where {}", where_used.join(", "),) + &format!(" (+{} more)", omitted)
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
                let mut var_declarations_exist = false;
                let var_map = func.variables.read().unwrap();
                let mut var_keys_sorted = var_map.keys().collect::<Vec<_>>();
                var_keys_sorted.sort_by_cached_key(|key| key.index);
                for var_key in var_keys_sorted {
                    let var = var_map.get(var_key).unwrap();
                    if let Some(const_value) = &var.const_value {
                        if !config.replace_constant {
                            var_declarations_exist = true;
                            let mut line = format!(
                                "  const {} {} = {};\n",
                                var.var_type.to_string_with_config(Some(config)),
                                var.name(),
                                const_value.to_string_with_config(Some(config))
                            );
                            if config.variable_usage_comment {
                                line.pop();
                                line.push_str(&format!(" /* {} */\n", variable_usage_summary(var)));
                            }
                            output.push_str(&line);
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
                        var_declarations_exist = true;
                        let mut line = format!(
                            "  {} {};\n",
                            var.var_type.to_string_with_config(Some(config)),
                            var.name()
                        );
                        if config.variable_usage_comment {
                            line.pop();
                            line.push_str(&format!(" /* {} */\n", variable_usage_summary(var)));
                        }
                        output.push_str(&line);
                    }
                }
                if var_declarations_exist {
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
                if config.print_instruction {
                    if let AstStatementOrigin::Ir(descriptor) = &stmt.origin
                        && !visited_ir.contains(&descriptor.descriptor().ir_index())
                    {
                        let instruction = &descriptor.ir().get_instructions()
                            [descriptor.descriptor().ir_index() as usize];
                        output.push_str(&format!("  // {}\n", instruction));
                        visited_ir.insert(descriptor.descriptor().ir_index());
                    }
                }
                if config.print_ir {
                    if let AstStatementOrigin::Ir(descriptor) = &stmt.origin
                        && let Some(statement_index) = descriptor.descriptor().statement_index()
                    {
                        if prev_stmt != Some(descriptor.descriptor()) {
                            let stmt = &descriptor.ir().get_ir()
                                [descriptor.descriptor().ir_index() as usize]
                                .statements
                                .as_ref()
                                .unwrap()[*statement_index as usize];
                            output.push_str(&format!("    /* {} */\n", stmt));
                            prev_stmt = Some(descriptor.descriptor());
                        }
                    }
                }
                output.push_str(&format!("    {}\n", content));
            }

            output.push_str("}\n\n");
        }

        output.pop();
        output
    }
}
