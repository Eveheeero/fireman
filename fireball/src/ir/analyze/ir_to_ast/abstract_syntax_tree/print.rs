use super::*;
use hashbrown::HashSet;

impl Ast {
    pub fn print(&self, config: Option<AstPrintConfig>) -> String {
        let config = config.unwrap_or_default();
        let mut output = String::new();
        let function_versions = &self.function_versions;

        // Functions
        for (func_id, version_map) in self.functions.read().unwrap().iter() {
            let version = function_versions.get(func_id).unwrap();
            let func = version_map.get(version).unwrap();
            output.push_str(&format!(
                "{} {}(",
                func.return_type.to_string_with_config(Some(config)),
                func.name
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
                                var.name,
                                const_value.to_string_with_config(Some(config))
                            )
                        } else {
                            format!(
                                "{} {};\n",
                                var.var_type.to_string_with_config(Some(config)),
                                var.name
                            )
                        }
                    })
                    .collect();
                output.push_str(&params.join(", "));
            }

            output.push_str(") {\n");

            // Local variables
            for var in func.variables.read().unwrap().values() {
                if let Some(const_value) = &var.const_value {
                    output.push_str(&format!(
                        "const {} {} = {};\n",
                        var.var_type.to_string_with_config(Some(config)),
                        var.name,
                        const_value.to_string_with_config(Some(config))
                    ));
                } else {
                    output.push_str(&format!(
                        "{} {};\n",
                        var.var_type.to_string_with_config(Some(config)),
                        var.name
                    ));
                }
            }
            output.push_str("\n");

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
                        let instruction = &descriptor.ir.get_instructions()
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
                            let stmt = &descriptor.ir.get_ir()
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

        output
    }
}
