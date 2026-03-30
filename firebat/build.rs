#[cfg(windows)]
use std::env;

#[cfg(windows)]
extern crate winres;

fn main() {
    #[cfg(windows)]
    {
        let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
        if target_os == "windows" {
            // Embed Windows icon into executable
            let mut res = winres::WindowsResource::new();
            res.set_icon("icons/icon.ico");
            res.set_language(0x0409); // English (US)
            res.compile().expect("Failed to compile Windows resources");
        }
    }
    
    // Re-run if icon files change
    println!("cargo:rerun-if-changed=icons/icon.ico");
    println!("cargo:rerun-if-changed=icons/icon.icns");
    println!("cargo:rerun-if-changed=icons/icon.png");
}
