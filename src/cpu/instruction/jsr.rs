use crate::cpu::{Cpu, StateType};
use anyhow::{bail, Result};

impl Cpu {
    pub(in super::super) async fn jsr(&mut self, opcode: u16) -> Result<u8> {
        match (opcode >> 8) as u8 {
            0x5d => return self.jsr_ern(opcode).await,
            0x5e => return self.jsr_abs(opcode).await,
            0x5f => return self.jsr_indirect(opcode).await,
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
    }

    async fn jsr_ern(&mut self, opcode: u16) -> Result<u8> {
        let access_addr = (self.read_rn_l(7)? - 4) & 0x00ffffff;
        self.write_dec_ern_l(7, self.pc).await?;
        let addr = self.read_rn_l(Cpu::get_nibble_opcode(opcode, 3)?)?;
        self.pc = addr;
        Ok(self.calc_state(StateType::I, 2).await?
            + self
                .calc_state_with_addr(StateType::K, 2, access_addr)
                .await?)
    }

    async fn jsr_abs(&mut self, opcode: u16) -> Result<u8> {
        let access_addr = (self.read_rn_l(7)? - 4) & 0x00ffffff;
        let opcode2 = self.fetch().await;
        let abs_addr = (((opcode & 0x00ff) as u32) << 16) | opcode2 as u32;
        self.write_dec_ern_l(7, self.pc).await?;
        self.pc = abs_addr;
        Ok(self.calc_state(StateType::I, 2).await?
            + self
                .calc_state_with_addr(StateType::K, 2, access_addr)
                .await?
            + self.calc_state(StateType::N, 2).await?)
    }

    async fn jsr_indirect(&mut self, opcode: u16) -> Result<u8> {
        let abs_addr = (opcode & 0x00ff) as u8;
        self.write_dec_ern_l(7, self.pc).await?;
        let addr = self.read_abs8_l(abs_addr).await?;
        self.pc = addr;
        let access_addr = self.get_addr_abs8(opcode as u8);
        Ok(self.calc_state(StateType::I, 2).await?
            + self
                .calc_state_with_addr(StateType::J, 2, access_addr)
                .await?
            + self.calc_state(StateType::K, 2).await?)
    }
}
