use fireball::{
    core::{Address, Block, FireRaw},
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
struct DecompileResult {
    assembly: Vec<Assembly>,
    ir: Vec<Ir>,
    decompiled: String,
}
#[derive(Serialize, TsBind)]
#[ts_bind(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
struct Assembly {
    index: usize,
    parents_start_address: u64,
    data: String,
}
#[derive(Serialize, TsBind)]
#[ts_bind(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
struct Ir {
    parents_assembly_index: usize,
    data: String,
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

#[tauri::command(rename_all = "snake_case")]
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

#[tauri::command(rename_all = "snake_case")]
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

#[tauri::command(rename_all = "snake_case")]
fn decompile_sections(start_addresses: Vec<u64>) -> Result<DecompileResult, String> {
    let mut assembly = Vec::new();
    let mut irs = Vec::new();
    let app = APP.read().unwrap();
    let fireball = app.fireball()?;
    let blocks = fireball.get_blocks();
    let sections = fireball.get_sections();
    let target_blocks = start_addresses
        .iter()
        .map(|&addr| Address::from_virtual_address(&sections, addr))
        .filter_map(|addr| blocks.get_by_start_address(&addr))
        .collect::<Vec<_>>();
    let mut assembly_index = 0;
    for target_block in &target_blocks {
        let start_address = target_block.get_start_address().get_virtual_address();
        let ir = target_block.get_ir();
        let Some(ir) = ir.as_ref() else {
            continue;
        };
        for (instruction, ir) in ir.instructions().iter().zip(ir.ir()) {
            assembly_index += 1;
            assembly.push(Assembly {
                index: assembly_index,
                parents_start_address: start_address,
                data: instruction.to_string(),
            });
            let Some(statements) = ir.statements.as_ref() else {
                continue;
            };
            for statement in statements.iter() {
                irs.push(Ir {
                    parents_assembly_index: assembly_index,
                    data: statement.to_string(),
                });
            }
        }
    }
    let decompiled = fireball::ir::analyze::generate_c(target_blocks).to_c_code();
    Ok(DecompileResult {
        assembly,
        ir: irs,
        decompiled,
    })
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            analyze_section,
            open_file,
            decompile_sections
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
