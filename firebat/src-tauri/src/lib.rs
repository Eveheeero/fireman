use fireball::{
    core::{Address, Block, FireRaw},
    ir::utils::IrStatementDescriptor,
    Fireball,
};
use serde::Serialize;
use std::sync::{Arc, LazyLock, RwLock};
use ts_bind::TsBind;

struct Firebat {
    path: Option<String>,
    fireball: Option<Fireball>,
}
unsafe impl Send for Firebat {}
unsafe impl Sync for Firebat {}
impl Firebat {
    fn new() -> Self {
        Self {
            path: None,
            fireball: None,
        }
    }
    fn fireball(&self) -> Result<&Fireball, String> {
        if let Some(fireball) = &self.fireball {
            Ok(fireball)
        } else {
            Err("Fireball is None".to_string())
        }
    }
}

static APP: LazyLock<Arc<RwLock<Firebat>>> =
    LazyLock::new(|| Arc::new(RwLock::new(Firebat::new())));

#[derive(Serialize, TsBind)]
#[ts_bind(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
struct KnownSection {
    start_address: u64,
    end_address: Option<u64>,
    analyzed: bool,
}
#[derive(Serialize, TsBind)]
#[ts_bind(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
struct IrInspectResult {
    instruction: String,
    statements: Vec<IrInspectResultSingle>,
}
#[derive(Serialize, TsBind)]
#[ts_bind(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
struct IrInspectResultSingle {
    statement: String,
    data_accesses: Vec<String>,
    data_access_per_ir: Vec<String>,
}

fn parse_address(address: &str) -> Result<u64, String> {
    let address = address.trim();
    if let Ok(address) = address.parse::<u64>() {
        return Ok(address);
    }
    let address = if address.starts_with("0x") || address.starts_with("0X") {
        &address[2..]
    } else {
        address
    };
    if let Ok(address) = u64::from_str_radix(address, 16) {
        return Ok(address);
    }
    Err("Invalid Address".to_string())
}

#[tauri::command]
fn open_file(path: &str) -> Result<(), String> {
    let mut app = APP.write().unwrap();
    app.path = Some(path.to_owned());
    let fireball = Fireball::from_path(path);
    if let Err(e) = fireball {
        return Err(e.to_string());
    }
    app.fireball = Some(fireball.unwrap());
    Ok(())
}

#[tauri::command]
fn analyze_section(address: &str) -> Result<Vec<KnownSection>, String> {
    if address.is_empty() {
        return analyze_section_from_entry();
    }
    let address = parse_address(address)?;
    analyze_section_from_address(address)
}

fn analyze_section_from_address(address: u64) -> Result<Vec<KnownSection>, String> {
    let app = APP.read().unwrap();
    let fireball = app.fireball()?;
    let result = fireball.analyze_from_virtual_address(address);
    if let Err(e) = result {
        return Err(e.to_string());
    }
    let result = result.unwrap();
    Ok(block_to_result(result))
}
fn analyze_section_from_entry() -> Result<Vec<KnownSection>, String> {
    let app = APP.read().unwrap();
    let fireball = app.fireball()?;
    let result = fireball.analyze_from_entry();
    if let Err(e) = result {
        return Err(e.to_string());
    }
    let result = result.unwrap();
    Ok(block_to_result(result))
}
fn block_to_result(block: Arc<Block>) -> Vec<KnownSection> {
    let reader = block.get_connected_to();
    let mut result = Vec::new();
    let start_address = block.get_start_address().get_virtual_address();
    let o = KnownSection {
        end_address: block.get_block_size().map(|a| start_address + a),
        start_address,
        analyzed: true,
    };
    result.push(o);

    for i in reader.iter() {
        let Some(to) = i.to() else {
            continue;
        };
        result.push(KnownSection {
            start_address: to.get_virtual_address(),
            end_address: None,
            analyzed: false,
        });
    }
    result
}

#[tauri::command]
fn ir_inspect(address: &str) -> Result<Vec<IrInspectResult>, String> {
    let app = APP.read().unwrap();
    let fireball = app.fireball()?;
    let address = parse_address(address)?;
    let sections = fireball.get_sections();
    let address = Address::from_virtual_address(&sections, address);
    let block = fireball.get_blocks().get_by_containing_address(&address);
    let block = &block.get(0).ok_or("Block Not Analyzed")?;
    let ir_block = block.get_ir();
    let ir_block = ir_block.as_ref().ok_or("Block Not Analyzed")?;
    let data_access = ir_block.data_access.as_ref().ok_or("Block Not Analyzed")?;
    let known_datatypes = ir_block
        .known_datatypes
        .as_ref()
        .ok_or("Block Not Analyzed")?;
    let mut result = Vec::new();
    for (ir_index, (ir, instruction)) in ir_block
        .ir()
        .iter()
        .zip(ir_block.instructions().iter())
        .enumerate()
    {
        if let Some(statements) = ir.statements {
            for (statement_index, statement) in statements.iter().enumerate() {
                let key = IrStatementDescriptor::new(ir_index as u32, statement_index as u8);
                let data_access = data_access.get(key).unwrap();
                let known_datatypes = known_datatypes.get(key).unwrap();
                result.push(IrInspectResult {
                    instruction: format!("{}", instruction),
                    statements: vec![IrInspectResultSingle {
                        statement: format!("{}", statement),
                        data_accesses: data_access.iter().map(|s| format!("{}", s)).collect(),
                        data_access_per_ir: known_datatypes
                            .iter()
                            .map(|s| format!("{}", s))
                            .collect(),
                    }],
                });
            }
        } else {
            result.push(IrInspectResult {
                instruction: format!("{}", instruction),
                statements: vec![],
            });
        }
    }
    Ok(result)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            analyze_section,
            open_file,
            ir_inspect
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
