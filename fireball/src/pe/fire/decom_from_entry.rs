use super::PE;
use crate::core::Address;

impl PE {
    pub(super) fn _decom_from_entry(&self) -> Result<(), Box<dyn std::error::Error>> {
        let gl = goblin::pe::PE::parse(&self.binary)?;
        let entry = Address::from_virtual_address(&self.sections, gl.entry as u64).unwrap();

        let insts = self.parse_assem_range(entry, 0x100).unwrap();

        for inst in insts.iter() {
            println!("{}", inst);
            if inst.mnemonic().unwrap() == "call" {
                let op = inst.op_str().unwrap();
                if op.starts_with("0x") {
                    let op_ptr = u64::from_str_radix(op.trim_start_matches("0x"), 16)?;
                    for now in self.defined.iter() {
                        if now.address.get_virtual_address() == op_ptr {
                            println!("{}: {}", now.name, now.address.get_virtual_address());
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
