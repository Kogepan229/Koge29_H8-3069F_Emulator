use crate::cpu::Cpu;
use anyhow::{bail, Context as _, Result};

impl<'a> Cpu<'a> {
    pub(in super::super) fn jsr(&mut self, opcode: u16) -> Result<usize> {
        match (opcode >> 8) as u8 {
            0x5d => return self.jsr_ern(opcode),
            0x5e => return self.jsr_abs(opcode),
            0x5f => return self.jsr_indirect(opcode),
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
    }

    fn jsr_ern(&mut self, opcode: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            self.write_dec_ern_l(7, self.pc)?;
            let addr = self.read_rn_l(Cpu::get_nibble_opcode(opcode, 3)?)?;
            self.pc = addr;
            Ok(8)
        };
        f()
    }

    fn jsr_abs(&mut self, opcode: u16) -> Result<usize> {
        let opcode2 = self.fetch();
        let abs_addr = (((opcode & 0x00ff) as u32) << 16) | opcode2 as u32;
        let mut f = || -> Result<usize> {
            self.write_dec_ern_l(7, self.pc)?;
            self.pc = abs_addr;
            Ok(10)
        };
        f().with_context(|| format!("opcode2:[{:x}] abs_addr:[{:x}]", opcode2, abs_addr))
    }

    fn jsr_indirect(&mut self, opcode: u16) -> Result<usize> {
        let abs_addr = (opcode & 0x00ff) as u8;
        let mut f = || -> Result<usize> {
            self.write_dec_ern_l(7, self.pc)?;
            let addr = self.read_abs8_l(abs_addr)?;
            self.pc = addr;
            Ok(12)
        };
        f().with_context(|| format!("abs_addr:[{:x}]", abs_addr))
    }
}
