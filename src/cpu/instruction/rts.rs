use crate::cpu::{Cpu, StateType};
use anyhow::Result;

impl Cpu {
    pub(in super::super) async fn rts(&mut self) -> Result<u8> {
        let access_addr = self.read_rn_l(7)? & 0x00ffffff;
        self.pc = self.read_inc_ern_l(7).await?;
        Ok(self.calc_state(StateType::I, 2).await?
            + self
                .calc_state_with_addr(StateType::K, 2, access_addr)
                .await?
            + self.calc_state(StateType::N, 2).await?)
    }
}
