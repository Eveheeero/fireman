use fireball::{core::Address, Fire, Fireball};
use serde::Serialize;
use std::sync::{Arc, LazyLock, Mutex};
use ts_bind::TsBind;

struct Firebat {
    path: Option<String>,
    fireball: Option<Fireball>,
}
unsafe impl Send for Firebat {}
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

static APP: LazyLock<Arc<Mutex<Firebat>>> = LazyLock::new(|| Arc::new(Mutex::new(Firebat::new())));

fn parse_address(address: &str) -> Result<u64, String> {
    if let Ok(address) = address.parse::<u64>() {
        Ok(address)
    } else if let Ok(address) = u64::from_str_radix(address, 16) {
        Ok(address)
    } else {
        return Err("Invalid Address".to_string());
    }
}

#[tauri::command]
fn open_file(path: &str) -> Result<(), String> {
    let mut app = APP.lock().unwrap();
    app.path = Some(path.to_owned());
    let fireball = Fireball::from_path(path);
    if let Err(e) = fireball {
        return Err(e.to_string());
    }
    app.fireball = Some(fireball.unwrap());
    Ok(())
}

#[tauri::command]
fn decom_from_entry() -> Result<Vec<u64>, String> {
    let app = APP.lock().unwrap();
    let fireball = app.fireball()?;
    let result = fireball.decom_from_entry();
    if let Err(e) = result {
        return Err(e.to_string());
    }
    let result = result.unwrap();
    let reader = result.get_connected_to();
    let mut connected_to = Vec::new();
    for i in reader.iter() {
        let Some(to) = i.to() else {
            continue;
        };
        connected_to.push(to.get_virtual_address());
    }
    Ok(connected_to)
}

#[tauri::command]
fn decom_from_address(address: &str) -> Result<Vec<u64>, String> {
    let app = APP.lock().unwrap();
    let fireball = app.fireball()?;
    let address = parse_address(address)?;
    let result = fireball.decom_from_virtual_address(address);
    if let Err(e) = result {
        return Err(e.to_string());
    }
    let result = result.unwrap();
    let reader = result.get_connected_to();
    let mut connected_to = Vec::new();
    for i in reader.iter() {
        let Some(to) = i.to() else {
            continue;
        };
        connected_to.push(to.get_virtual_address());
    }
    Ok(connected_to)
}

#[derive(Serialize, TsBind)]
struct IrInspectResult {
    instruction: String,
    statements: Option<Vec<String>>,
    data_accesses: Option<Vec<String>>,
    data_access_per_ir: Option<Vec<String>>,
}
#[tauri::command]
fn ir_inspect(address: &str) -> Result<Vec<IrInspectResult>, String> {
    let app = APP.lock().unwrap();
    let fireball = app.fireball()?;
    let address = parse_address(address)?;
    let sections = fireball.get_sections();
    let address = Address::from_virtual_address(&sections, address);
    let block = fireball.get_blocks().get_by_containing_address(&address);
    let block = &block.get(0).ok_or("Block Not Analyzed")?;
    let ir_block = block.get_ir();
    let ir_block = ir_block.as_ref().ok_or("Block Not Analyzed")?;
    let data_access_per_ir = ir_block
        .data_access_per_ir
        .as_ref()
        .ok_or("Block Not Analyzed")?;
    let known_datatypes_per_ir = ir_block
        .known_datatypes_per_ir
        .as_ref()
        .ok_or("Block Not Analyzed")?;
    let mut result = Vec::new();
    for (i, ir) in ir_block.ir().iter().enumerate() {
        let instruction = ir.instruction.as_ref();
        if let Some(statements) = ir.statements {
            let data_access = data_access_per_ir.get(i).unwrap();
            let known_datatypes = known_datatypes_per_ir.get(i).unwrap();
            result.push(IrInspectResult {
                instruction: format!("{:?}", instruction),
                statements: Some(statements.iter().map(|s| format!("{:?}", s)).collect()),
                data_accesses: Some(data_access.iter().map(|s| format!("{:?}", s)).collect()),
                data_access_per_ir: Some(
                    known_datatypes.iter().map(|s| format!("{:?}", s)).collect(),
                ),
            });
        } else {
            result.push(IrInspectResult {
                instruction: format!("{:?}", instruction),
                statements: None,
                data_accesses: None,
                data_access_per_ir: None,
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
            decom_from_address,
            open_file,
            decom_from_entry,
            ir_inspect
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
