use crate::cpu::Cpu;
use anyhow::Result;

impl<'a> Cpu<'a> {
    pub(in super::super) fn adds1(&mut self, opcode: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            let register = Cpu::get_nibble_opcode(opcode, 4)?;
            let value = self.read_rn_l(register)?;
            self.write_rn_l(register, value + 1)?;
            Ok(2)
        };
        f()
    }
    pub(in super::super) fn adds2(&mut self, opcode: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            let register = Cpu::get_nibble_opcode(opcode, 4)?;
            let value = self.read_rn_l(register)?;
            self.write_rn_l(register, value + 2)?;
            Ok(2)
        };
        f()
    }
    pub(in super::super) fn adds4(&mut self, opcode: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            let register = Cpu::get_nibble_opcode(opcode, 4)?;
            let value = self.read_rn_l(register)?;
            self.write_rn_l(register, value + 4)?;
            Ok(2)
        };
        f()
    }
}
