use crate::cpu::Cpu;
use anyhow::Result;

impl Cpu {
    pub(in super::super) fn subs1(&mut self, opcode: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            let register = Cpu::get_nibble_opcode(opcode, 4)?;
            let value = self.read_rn_l(register)?;
            self.write_rn_l(register, value.wrapping_add_signed(-1))?;
            Ok(2)
        };
        f()
    }

    pub(in super::super) fn subs2(&mut self, opcode: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            let register = Cpu::get_nibble_opcode(opcode, 4)?;
            let value = self.read_rn_l(register)?;
            self.write_rn_l(register, value.wrapping_add_signed(-2))?;
            Ok(2)
        };
        f()
    }

    pub(in super::super) fn subs4(&mut self, opcode: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            let register = Cpu::get_nibble_opcode(opcode, 4)?;
            let value = self.read_rn_l(register)?;
            self.write_rn_l(register, value.wrapping_add_signed(-4))?;
            Ok(2)
        };
        f()
    }
}
