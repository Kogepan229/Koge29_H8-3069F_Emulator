use crate::cpu::{Cpu, StateType, CCR};
use anyhow::{Context as _, Result};

impl Cpu {
    pub(in super::super) async fn or_b_imm(&mut self, opcode: u16) -> Result<u8> {
        let register = Cpu::get_nibble_opcode(opcode, 2)?;
        let result = self.read_rn_b(register)? | opcode as u8;
        self.write_rn_b(register, result)?;
        if (result as i8) < 0 {
            self.write_ccr(CCR::N, 1);
        } else {
            self.write_ccr(CCR::N, 0);
        }
        if result == 0 {
            self.write_ccr(CCR::Z, 1);
        } else {
            self.write_ccr(CCR::Z, 0);
        }
        self.write_ccr(CCR::V, 0);
        Ok(self.calc_state(StateType::I, 1).await?)
    }

    pub(in super::super) async fn or_b_rn(&mut self, opcode: u16) -> Result<u8> {
        let register_src = Cpu::get_nibble_opcode(opcode, 3)?;
        let register_dest = Cpu::get_nibble_opcode(opcode, 4)?;
        let result = self.read_rn_b(register_src)? | self.read_rn_b(register_dest)?;
        self.write_rn_b(register_dest, result)?;
        if (result as i8) < 0 {
            self.write_ccr(CCR::N, 1);
        } else {
            self.write_ccr(CCR::N, 0);
        }
        if result == 0 {
            self.write_ccr(CCR::Z, 1);
        } else {
            self.write_ccr(CCR::Z, 0);
        }
        self.write_ccr(CCR::V, 0);
        Ok(self.calc_state(StateType::I, 1).await?)
    }

    pub(in super::super) async fn or_w_imm(&mut self, opcode: u16) -> Result<u8> {
        let opcode2 = self.fetch().await;

        let mut f = || -> Result<()> {
            let register = Cpu::get_nibble_opcode(opcode, 4)?;
            let result = self.read_rn_w(register)? | opcode2;
            self.write_rn_w(register, result)?;
            if (result as i16) < 0 {
                self.write_ccr(CCR::N, 1);
            } else {
                self.write_ccr(CCR::N, 0);
            }
            if result == 0 {
                self.write_ccr(CCR::Z, 1);
            } else {
                self.write_ccr(CCR::Z, 0);
            }
            self.write_ccr(CCR::V, 0);
            return Ok(());
        };
        f().with_context(|| format!("opcode2(imm) [{:x}]", opcode2))?;
        Ok(self.calc_state(StateType::I, 2).await?)
    }

    pub(in super::super) async fn or_w_rn(&mut self, opcode: u16) -> Result<u8> {
        let register_src = Cpu::get_nibble_opcode(opcode, 3)?;
        let register_dest = Cpu::get_nibble_opcode(opcode, 4)?;
        let result = self.read_rn_w(register_src)? | self.read_rn_w(register_dest)?;
        self.write_rn_w(register_dest, result)?;
        if (result as i16) < 0 {
            self.write_ccr(CCR::N, 1);
        } else {
            self.write_ccr(CCR::N, 0);
        }
        if result == 0 {
            self.write_ccr(CCR::Z, 1);
        } else {
            self.write_ccr(CCR::Z, 0);
        }
        self.write_ccr(CCR::V, 0);
        Ok(self.calc_state(StateType::I, 1).await?)
    }

    pub(in super::super) async fn or_l_imm(&mut self, opcode: u16) -> Result<u8> {
        let imm = ((self.fetch().await as u32) << 16) | self.fetch().await as u32;

        let mut f = || -> Result<()> {
            let register = Cpu::get_nibble_opcode(opcode, 4)?;
            let result = self.read_rn_l(register)? | imm;
            self.write_rn_l(register, result)?;
            if (result as i32) < 0 {
                self.write_ccr(CCR::N, 1);
            } else {
                self.write_ccr(CCR::N, 0);
            }
            if result == 0 {
                self.write_ccr(CCR::Z, 1);
            } else {
                self.write_ccr(CCR::Z, 0);
            }
            self.write_ccr(CCR::V, 0);
            return Ok(());
        };
        f().with_context(|| format!("imm(opcode2, 3) [{:x}]", imm))?;
        Ok(self.calc_state(StateType::I, 3).await?)
    }

    pub(in super::super) async fn or_l_rn(&mut self, _opcode: u16, opcode2: u16) -> Result<u8> {
        let mut f = || -> Result<()> {
            let register_src = Cpu::get_nibble_opcode(opcode2, 3)?;
            let register_dest = Cpu::get_nibble_opcode(opcode2, 4)?;
            let result = self.read_rn_l(register_src)? | self.read_rn_l(register_dest)?;
            self.write_rn_l(register_dest, result)?;
            if (result as i32) < 0 {
                self.write_ccr(CCR::N, 1);
            } else {
                self.write_ccr(CCR::N, 0);
            }
            if result == 0 {
                self.write_ccr(CCR::Z, 1);
            } else {
                self.write_ccr(CCR::Z, 0);
            }
            self.write_ccr(CCR::V, 0);
            return Ok(());
        };
        f().with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        Ok(self.calc_state(StateType::I, 2).await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::{cpu::Cpu, memory::MEMORY_START_ADDR};

    #[tokio::test]
    async fn test_or_b_imm() {
        let mut cpu = Cpu::new();

        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0xc0, 0x30]);
        cpu.write_rn_b(0, 0xaf).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0xbf);

        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0xcf, 0x30]);
        cpu.write_rn_b(0xf, 0xaf).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_b(0xf).unwrap(), 0xbf);

        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0xc0, 0x00]);
        cpu.write_rn_b(0, 0).unwrap();
        let opcode = cpu.fetch().await;
        cpu.exec(opcode).await.unwrap();
        assert_eq!(cpu.ccr & 0b00001110, 0b00000100);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0);
    }

    #[tokio::test]
    async fn test_or_b_rn() {
        let mut cpu = Cpu::new();

        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x14, 0x0f]);
        cpu.write_rn_b(0, 0xaf).unwrap();
        cpu.write_rn_b(0xf, 0x30).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        // check result
        assert_eq!(cpu.read_rn_b(0xf).unwrap(), 0xbf);
        // check no change
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0xaf);

        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x14, 0xf0]);
        cpu.write_rn_b(0xf, 0xaf).unwrap();
        cpu.write_rn_b(0, 0x30).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        // check result
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0xbf);
        // check no change
        assert_eq!(cpu.read_rn_b(0xf).unwrap(), 0xaf);

        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x14, 0x0f]);
        cpu.write_rn_b(0xf, 0).unwrap();
        cpu.write_rn_b(0, 0).unwrap();
        let opcode = cpu.fetch().await;
        cpu.exec(opcode).await.unwrap();
        assert_eq!(cpu.ccr & 0b00001110, 0b00000100);
        assert_eq!(cpu.read_rn_b(0xf).unwrap(), 0);
    }

    #[tokio::test]
    async fn test_or_w_imm() {
        let mut cpu = Cpu::new();

        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x79, 0x40, 0x30, 0x30]);
        cpu.write_rn_w(0, 0xafaf).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 4);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0xbfbf);

        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x79, 0x4f, 0x30, 0x30]);
        cpu.write_rn_w(0xf, 0xafaf).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 4);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0xbfbf);

        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x79, 0x40, 0x00, 0x00]);
        cpu.write_rn_w(0, 0).unwrap();
        let opcode = cpu.fetch().await;
        cpu.exec(opcode).await.unwrap();
        assert_eq!(cpu.ccr & 0b00001110, 0b00000100);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0);
    }

    #[tokio::test]
    async fn test_or_w_rn() {
        let mut cpu = Cpu::new();

        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x64, 0x0f]);
        cpu.write_rn_w(0, 0xafaf).unwrap();
        cpu.write_rn_w(0xf, 0x3030).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();

        assert_eq!(state, 2);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        // check result
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0xbfbf);
        // check no change
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0xafaf);

        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x64, 0xf0]);
        cpu.write_rn_w(0xf, 0xafaf).unwrap();
        cpu.write_rn_w(0, 0x3030).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();

        assert_eq!(state, 2);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        // check result
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0xbfbf);
        // check no change
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0xafaf);

        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x64, 0x0f]);
        cpu.write_rn_w(0xf, 0).unwrap();
        cpu.write_rn_w(0, 0).unwrap();
        let opcode = cpu.fetch().await;
        cpu.exec(opcode).await.unwrap();
        assert_eq!(cpu.ccr & 0b00001110, 0b00000100);
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0);
    }

    #[tokio::test]
    async fn test_or_l_imm() {
        let mut cpu = Cpu::new();

        cpu.bus.lock().await.memory[0..6].copy_from_slice(&[0x7a, 0x40, 0x30, 0x30, 0x30, 0x30]);
        cpu.write_rn_l(0, 0xafafafaf).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0xbfbfbfbf);

        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.lock().await.memory[0..6].copy_from_slice(&[0x7a, 0x47, 0x30, 0x30, 0x30, 0x30]);
        cpu.write_rn_l(0x7, 0xafafafaf).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_l(0x7).unwrap(), 0xbfbfbfbf);

        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.lock().await.memory[0..6].copy_from_slice(&[0x7a, 0x40, 0x00, 0x00, 0x00, 0x00]);
        cpu.write_rn_l(0, 0).unwrap();
        let opcode = cpu.fetch().await;
        cpu.exec(opcode).await.unwrap();
        assert_eq!(cpu.ccr & 0b00001110, 0b00000100);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0);
    }

    #[tokio::test]
    async fn test_or_l_rn() {
        let mut cpu = Cpu::new();

        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x01, 0xf0, 0x64, 0x07]);
        cpu.write_rn_l(0, 0xafafafaf).unwrap();
        cpu.write_rn_l(7, 0x30303030).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 4);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        // check result
        assert_eq!(cpu.read_rn_l(7).unwrap(), 0xbfbfbfbf);
        // check no change
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0xafafafaf);

        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x01, 0xf0, 0x64, 0x70]);
        cpu.write_rn_l(7, 0xafafafaf).unwrap();
        cpu.write_rn_l(0, 0x30303030).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 4);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        // check result
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0xbfbfbfbf);
        // check no change
        assert_eq!(cpu.read_rn_l(7).unwrap(), 0xafafafaf);

        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x01, 0xf0, 0x64, 0x70]);
        cpu.write_rn_l(7, 0).unwrap();
        cpu.write_rn_l(0, 0).unwrap();
        let opcode = cpu.fetch().await;
        cpu.exec(opcode).await.unwrap();
        assert_eq!(cpu.ccr & 0b00001110, 0b00000100);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0);
    }
}
