use crate::cpu::Cpu;
use anyhow::{bail, Result};

impl<'a> Cpu<'a> {
    pub(in super::super) fn jmp(&mut self, opcode: u16) -> Result<usize> {
        match (opcode >> 8) as u8 {
            0x59 => return self.jmp_ern(opcode),
            0x5a => return self.jmp_abs(opcode),
            0x5b => return self.jmp_indirect(opcode),
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
    }

    fn jmp_ern(&mut self, opcode: u16) -> Result<usize> {
        let addr = self.read_rn_l(Cpu::get_nibble_opcode(opcode, 3)?)?;
        self.pc = addr;
        Ok(4)
    }
    fn jmp_abs(&mut self, opcode: u16) -> Result<usize> {
        let abs_addr = ((opcode & 0x00ff) as u32) << 16 | self.fetch() as u32;
        self.pc = abs_addr;
        Ok(6)
    }
    fn jmp_indirect(&mut self, opcode: u16) -> Result<usize> {
        let addr = self.read_abs8_l(opcode as u8)?;
        self.pc = addr;
        Ok(10)
    }
}
