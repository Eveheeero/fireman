use super::PE;
use crate::core::Address;

use capstone::Instructions;

impl PE {
    /// 범위만큼의 어셈블리 코드를 파싱한다.
    pub(crate) fn parse_assem_range(&self, offset: Address, size: u64) -> Result<Instructions, ()> {
        let file_offset = offset.get_file_offset();
        let virtual_offset = offset.get_virtual_address();
        let insns = match self.capstone.disasm_all(
            &self.binary[file_offset as usize..(file_offset + size) as usize],
            virtual_offset as u64,
        ) {
            Ok(insts) => insts,
            Err(_) => return Err(()),
        };
        Ok(insns)
    }

    /// 어셈블리 코드를 N개 파싱한다.
    pub(crate) fn parse_assem_count(
        &self,
        offset: Address,
        count: usize,
    ) -> Result<Instructions, ()> {
        let file_offset = offset.get_file_offset();
        let virtual_offset = offset.get_virtual_address();
        let insns = match self.capstone.disasm_count(
            &self.binary[file_offset as usize..],
            virtual_offset as u64,
            count,
        ) {
            Ok(insts) => insts,
            Err(_) => return Err(()),
        };
        Ok(insns)
    }
}
