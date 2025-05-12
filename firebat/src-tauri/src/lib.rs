use fireball::{Fire, Fireball};
use std::sync::{Arc, LazyLock, Mutex};

struct Firebat {
    path: Option<String>,
    fireball: Option<Fireball>,
}
unsafe impl Send for Firebat {}
impl Firebat {
    pub fn new() -> Self {
        Self {
            path: None,
            fireball: None,
        }
    }
}

static APP: LazyLock<Arc<Mutex<Firebat>>> = LazyLock::new(|| Arc::new(Mutex::new(Firebat::new())));

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
    let Some(fireball) = app.fireball.as_ref() else {
        return Err("fireball is None".to_string());
    };
    let result = fireball.decom_from_entry();
    if let Err(e) = result {
        return Err(e.to_string());
    }
    let result = result.unwrap();
    let reader = result.get_connected_to();
    let mut connected_to = Vec::new();
    for i in reader.iter() {
        if i.to().is_none() {
            continue;
        }
        let to = &i.to().unwrap();
        connected_to.push(to.get_virtual_address());
    }
    Ok(connected_to)
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, open_file, decom_from_entry])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
