use crate::cpu::{Cpu, StateType, CCR};
use anyhow::{bail, Context as _, Result};

impl Cpu {
    pub(in super::super) async fn bcc(&mut self, opcode: u16) -> Result<u8> {
        match (opcode >> 8) as u8 {
            0x40 => return self.bra8(opcode).await,
            0x41 => return self.brn8().await,
            0x42 => return self.bhi8(opcode).await,
            0x43 => return self.bls8(opcode).await,
            0x44 => return self.bcc8(opcode).await,
            0x45 => return self.bcs8(opcode).await,
            0x46 => return self.bne8(opcode).await,
            0x47 => return self.beq8(opcode).await,
            0x48 => return self.bvc8(opcode).await,
            0x49 => return self.bvs8(opcode).await,
            0x4a => return self.bpl8(opcode).await,
            0x4b => return self.bmi8(opcode).await,
            0x4c => return self.bge8(opcode).await,
            0x4d => return self.blt8(opcode).await,
            0x4e => return self.bgt8(opcode).await,
            0x4f => return self.ble8(opcode).await,
            0x58 => match opcode as u8 {
                0x00 => return self.bra16().await,
                0x10 => return self.brn16().await,
                0x20 => return self.bhi16().await,
                0x30 => return self.bls16().await,
                0x40 => return self.bcc16().await,
                0x50 => return self.bcs16().await,
                0x60 => return self.bne16().await,
                0x70 => return self.beq16().await,
                0x80 => return self.bvc16().await,
                0x90 => return self.bvs16().await,
                0xa0 => return self.bpl16().await,
                0xb0 => return self.bmi16().await,
                0xc0 => return self.bge16().await,
                0xd0 => return self.blt16().await,
                0xe0 => return self.bgt16().await,
                0xf0 => return self.ble16().await,
                _ => bail!("invalid opcode [{:>04x}]", opcode),
            },
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
    }

    async fn bra8(&mut self, opcode: u16) -> Result<u8> {
        self.pc_disp8(opcode as u8)?;
        Ok(self.calc_state(StateType::I, 2).await?)
    }

    async fn bra16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch().await;
        self.pc_disp16(opcode2)
            .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        Ok(self.calc_state(StateType::I, 2).await? + self.calc_state(StateType::N, 2).await?)
    }

    async fn brn8(&mut self) -> Result<u8> {
        Ok(self.calc_state(StateType::I, 2).await?)
    }

    async fn brn16(&mut self) -> Result<u8> {
        let _opcode2 = self.fetch().await;
        Ok(self.calc_state(StateType::I, 2).await? + self.calc_state(StateType::N, 2).await?)
    }

    async fn bhi8(&mut self, opcode: u16) -> Result<u8> {
        if self.read_ccr(CCR::C) | self.read_ccr(CCR::Z) == 0 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2).await?)
    }

    async fn bhi16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch().await;
        if self.read_ccr(CCR::C) | self.read_ccr(CCR::Z) == 0 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2).await? + self.calc_state(StateType::N, 2).await?)
    }

    async fn bls8(&mut self, opcode: u16) -> Result<u8> {
        if self.read_ccr(CCR::C) | self.read_ccr(CCR::Z) == 1 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2).await?)
    }

    async fn bls16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch().await;
        if self.read_ccr(CCR::C) | self.read_ccr(CCR::Z) == 1 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2).await? + self.calc_state(StateType::N, 2).await?)
    }

    async fn bcc8(&mut self, opcode: u16) -> Result<u8> {
        if self.read_ccr(CCR::C) == 0 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2).await?)
    }

    async fn bcc16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch().await;
        if self.read_ccr(CCR::C) == 0 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2).await? + self.calc_state(StateType::N, 2).await?)
    }

    async fn bcs8(&mut self, opcode: u16) -> Result<u8> {
        if self.read_ccr(CCR::C) == 1 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2).await?)
    }

    async fn bcs16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch().await;
        if self.read_ccr(CCR::C) == 1 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2).await? + self.calc_state(StateType::N, 2).await?)
    }

    async fn bne8(&mut self, opcode: u16) -> Result<u8> {
        if self.read_ccr(CCR::Z) == 0 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2).await?)
    }

    async fn bne16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch().await;
        if self.read_ccr(CCR::Z) == 0 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2).await? + self.calc_state(StateType::N, 2).await?)
    }

    async fn beq8(&mut self, opcode: u16) -> Result<u8> {
        if self.read_ccr(CCR::Z) == 1 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2).await?)
    }

    async fn beq16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch().await;
        if self.read_ccr(CCR::Z) == 1 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2).await? + self.calc_state(StateType::N, 2).await?)
    }

    async fn bvc8(&mut self, opcode: u16) -> Result<u8> {
        if self.read_ccr(CCR::V) == 0 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2).await?)
    }

    async fn bvc16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch().await;
        if self.read_ccr(CCR::V) == 0 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2).await? + self.calc_state(StateType::N, 2).await?)
    }

    async fn bvs8(&mut self, opcode: u16) -> Result<u8> {
        if self.read_ccr(CCR::V) == 1 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2).await?)
    }

    async fn bvs16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch().await;
        if self.read_ccr(CCR::V) == 1 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2).await? + self.calc_state(StateType::N, 2).await?)
    }

    async fn bpl8(&mut self, opcode: u16) -> Result<u8> {
        if self.read_ccr(CCR::N) == 0 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2).await?)
    }

    async fn bpl16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch().await;
        if self.read_ccr(CCR::N) == 0 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2).await? + self.calc_state(StateType::N, 2).await?)
    }

    async fn bmi8(&mut self, opcode: u16) -> Result<u8> {
        if self.read_ccr(CCR::N) == 1 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2).await?)
    }

    async fn bmi16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch().await;
        if self.read_ccr(CCR::N) == 1 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2).await? + self.calc_state(StateType::N, 2).await?)
    }

    async fn bge8(&mut self, opcode: u16) -> Result<u8> {
        if (self.read_ccr(CCR::N) ^ self.read_ccr(CCR::V)) == 0 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2).await?)
    }

    async fn bge16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch().await;
        if (self.read_ccr(CCR::N) ^ self.read_ccr(CCR::V)) == 0 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2).await? + self.calc_state(StateType::N, 2).await?)
    }

    async fn blt8(&mut self, opcode: u16) -> Result<u8> {
        if (self.read_ccr(CCR::N) ^ self.read_ccr(CCR::V)) == 1 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2).await?)
    }

    async fn blt16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch().await;
        if (self.read_ccr(CCR::N) ^ self.read_ccr(CCR::V)) == 1 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2).await? + self.calc_state(StateType::N, 2).await?)
    }

    async fn bgt8(&mut self, opcode: u16) -> Result<u8> {
        if self.read_ccr(CCR::Z) | (self.read_ccr(CCR::N) ^ self.read_ccr(CCR::V)) == 0 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2).await?)
    }

    async fn bgt16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch().await;
        if self.read_ccr(CCR::Z) | (self.read_ccr(CCR::N) ^ self.read_ccr(CCR::V)) == 0 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2).await? + self.calc_state(StateType::N, 2).await?)
    }

    async fn ble8(&mut self, opcode: u16) -> Result<u8> {
        if self.read_ccr(CCR::Z) | (self.read_ccr(CCR::N) ^ self.read_ccr(CCR::V)) == 1 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2).await?)
    }

    async fn ble16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch().await;
        if self.read_ccr(CCR::Z) | (self.read_ccr(CCR::N) ^ self.read_ccr(CCR::V)) == 1 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2).await? + self.calc_state(StateType::N, 2).await?)
    }
}
