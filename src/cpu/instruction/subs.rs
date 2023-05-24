use crate::cpu::Cpu;
use anyhow::{bail, Result};

impl<'a> Cpu<'a> {
    pub(in super::super) fn subs(&mut self, opcode: u16) -> Result<usize> {
        match Cpu::get_nibble_opcode(opcode, 3)? {
            0x0 => return self.subs1(opcode),
            0x8 => return self.subs2(opcode),
            0x9 => return self.subs4(opcode),
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
    }

    fn subs1(&mut self, opcode: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            let register = Cpu::get_nibble_opcode(opcode, 4)?;
            let value = self.read_rn_l(register)?;
            self.write_rn_l(register, value - 1)?;
            Ok(2)
        };
        f()
    }
    fn subs2(&mut self, opcode: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            let register = Cpu::get_nibble_opcode(opcode, 4)?;
            let value = self.read_rn_l(register)?;
            self.write_rn_l(register, value - 2)?;
            Ok(2)
        };
        f()
    }
    fn subs4(&mut self, opcode: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            let register = Cpu::get_nibble_opcode(opcode, 4)?;
            let value = self.read_rn_l(register)?;
            self.write_rn_l(register, value - 4)?;
            Ok(2)
        };
        f()
    }
}
