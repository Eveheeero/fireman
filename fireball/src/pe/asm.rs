use super::PE;

use capstone::Instructions;

impl PE {
    /// 어셈블리 코드를 파싱한다.
    pub(crate) fn parse_asm(&self, offset: usize, size: usize) -> Result<Instructions, ()> {
        let insns = match self
            .capstone
            .disasm_all(&self.binary[offset..offset + size], offset as u64)
        {
            Ok(insts) => insts,
            Err(_) => return Err(()),
        };
        Ok(insns)
    }
}
