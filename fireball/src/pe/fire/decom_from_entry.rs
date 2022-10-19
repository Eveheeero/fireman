use super::PE;
use crate::core::Address;

impl PE {
    pub(super) fn _decom_from_entry(&self) -> Result<(), Box<dyn std::error::Error>> {
        let gl = goblin::pe::PE::parse(&self.binary)?;
        let entry = Address::from_file_offset(&self.binary, gl.entry as u64);

        let insts = self.parse_assem_range(entry, 0x100).unwrap();

        for inst in insts.iter() {
            println!("{}", inst.to_string());
            if inst.mnemonic().unwrap() == "call" {
                let op = inst.op_str().unwrap();
                for now in self.defined.iter() {
                    if now.name == op {
                        println!("{}: {}", now.name, now.address.get_virtual_address());
                    }
                }
            }
        }

        Ok(())
    }
}
