use crate::ir::{analyze::MergedIr, statements::IrStatement};

pub fn generate_c(data: &MergedIr) -> String {
    let mut code = String::new();
    code.push_str("void decompiled_function() {\n");

    // Variable declarations
    for (i, var) in data.variables.iter().enumerate() {
        code.push_str(&format!("    {} v{};\n", var.data_type, i));
    }

    code.push('\n');

    // IR statements
    for (idx, ir) in data.ir.iter().enumerate() {
        code.push_str(&format!("    // IR[{}] at address {}\n", idx, ir.address));
        if let Some(stmts) = ir.statements {
            for stmt in stmts.iter() {
                match stmt {
                    IrStatement::Assignment { from, to, .. } => {
                        code.push_str(&format!("    {} = {};\n", to, from));
                    }
                    IrStatement::Call { target } => {
                        code.push_str(&format!("    {}();\n", target));
                    }
                    _ => {
                        code.push_str(&format!("    /* {} */\n", stmt));
                    }
                }
            }
        }
    }

    code.push_str("\n    return;\n}\n");
    code
}
