use std::io::Read;

fn main() {
    // Open Pe File and parse by goblin
    let mut file = std::fs::File::open("C:\\Windows\\System32\\kernel32.dll").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let pe = match goblin::pe::PE::parse(&buffer) {
        Ok(pe) => pe,
        Err(e) => panic!("Error: {}", e),
    };
    dbg!(pe);
}
