pub mod log;

pub fn parse_address(address: &str) -> Result<u64, String> {
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
    Err("Invalid Input".to_string())
}
