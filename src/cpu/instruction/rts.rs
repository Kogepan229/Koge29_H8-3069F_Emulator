use crate::cpu::Cpu;
use anyhow::Result;

impl Cpu {
    pub(in super::super) async fn rts(&mut self) -> Result<usize> {
        self.pc = self.read_inc_ern_l(7).await?;
        Ok(10)
    }
}
