use super::PE;
use crate::core::Address;

use capstone::Instructions;

impl PE {
    /// 어셈블리 코드를 파싱한다.
    pub(crate) fn parse_assem_range(
        &self,
        offset: Address,
        size: usize,
    ) -> Result<Instructions, ()> {
        let file_offset = offset.get_file_offset();
        let virtual_offset = offset.get_virtual_address();
        let insns = match self.capstone.disasm_all(
            &self.binary[file_offset..file_offset + size],
            virtual_offset as u64,
        ) {
            Ok(insts) => insts,
            Err(_) => return Err(()),
        };
        Ok(insns)
    }
}
