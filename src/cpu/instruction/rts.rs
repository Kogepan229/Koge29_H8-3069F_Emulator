use crate::cpu::Cpu;
use anyhow::Result;

impl<'a> Cpu<'a> {
    pub(in super::super) fn rts(&mut self) -> Result<usize> {
        self.pc = self.read_inc_ern_l(7)?;
        Ok(10)
    }
}
