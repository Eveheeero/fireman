pub mod c_abstract_syntax_tree;

use crate::ir::{analyze::MergedIr, statements::IrStatement};

pub fn generate_c(data: &MergedIr) -> String {
    let mut code = String::new();
    let mut indent = String::new();
    code.push_str("void decompiled_function() {\n");
    indent.push_str("    ");
    // Variable declarations
    for (i, var) in data.variables.iter().enumerate() {
        code.push_str(&format!("{indent}{} v{};\n", var.data_type, i));
    }

    code.push('\n');

    // IR statements
    for ir in data.ir.iter() {
        code.push_str(&format!("{indent}// {}\n", ir.instruction));
        if let Some(stmts) = ir.statements {
            for stmt in stmts.iter() {
                match stmt {
                    IrStatement::Assignment { from, to, .. } => {
                        code.push_str(&format!("{indent}{to} = {from};\n"));
                    }
                    IrStatement::Call { target } => {
                        code.push_str(&format!("{indent}{target}();\n"));
                    }
                    _ => {
                        code.push_str(&format!("{indent}/* {stmt} */\n"));
                    }
                }
            }
        }
    }

    code.push_str("}\n");
    code
}
