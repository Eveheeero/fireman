use super::*;
use hashbrown::HashSet;

impl Ast {
    pub fn print(&self, config: Option<AstPrintConfig>) -> String {
        let config = config.unwrap_or_default();
        let mut output = String::new();
        let function_versions = &self.function_versions;

        // Functions
        let functions = self.functions.read().unwrap();
        let mut function_keys_sorted = functions.keys().collect::<Vec<_>>();
        function_keys_sorted.sort_by_cached_key(|key| key.address);
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
                    .map(|var| {
                        if let Some(const_value) = &var.const_value {
                            format!(
                                "const {} {} = {};\n",
                                var.var_type.to_string_with_config(Some(config)),
                                var.name(),
                                const_value.to_string_with_config(Some(config))
                            )
                        } else {
                            format!(
                                "{} {};\n",
                                var.var_type.to_string_with_config(Some(config)),
                                var.name()
                            )
                        }
                    })
                    .collect();
                output.push_str(&params.join(", "));
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
                    var_declarations_exist = true;
                    if let Some(const_value) = &var.const_value {
                        output.push_str(&format!(
                            "  const {} {} = {};\n",
                            var.var_type.to_string_with_config(Some(config)),
                            var.name(),
                            const_value.to_string_with_config(Some(config))
                        ));
                    } else {
                        output.push_str(&format!(
                            "  {} {};\n",
                            var.var_type.to_string_with_config(Some(config)),
                            var.name()
                        ));
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
