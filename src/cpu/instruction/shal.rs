use crate::cpu::{Cpu, StateType, CCR};
use anyhow::Result;

impl Cpu {
    pub(in super::super) async fn shal_b(&mut self, opcode: u16) -> Result<u8> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let src = self.read_rn_b(register)?;
        if src & 0x40 == 0x40 {
            self.write_ccr(CCR::N, 1)
        } else {
            self.write_ccr(CCR::N, 0);
        }
        if (src << 1) == 0 {
            self.write_ccr(CCR::Z, 1)
        } else {
            self.write_ccr(CCR::Z, 0);
        }
        if src & 0x80 == 0x80 {
            self.write_ccr(CCR::V, 1)
        } else {
            self.write_ccr(CCR::V, 0);
        }
        self.write_ccr(CCR::C, src >> 7);
        self.write_rn_b(register, src << 1)?;

        Ok(self.calc_state(StateType::I, 1).await?)
    }

    pub(in super::super) async fn shal_w(&mut self, opcode: u16) -> Result<u8> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let src = self.read_rn_w(register)?;
        if src & 0x4000 == 0x4000 {
            self.write_ccr(CCR::N, 1)
        } else {
            self.write_ccr(CCR::N, 0);
        }
        if (src << 1) == 0 {
            self.write_ccr(CCR::Z, 1)
        } else {
            self.write_ccr(CCR::Z, 0);
        }
        if src & 0x8000 == 0x8000 {
            self.write_ccr(CCR::V, 1)
        } else {
            self.write_ccr(CCR::V, 0);
        }
        self.write_ccr(CCR::C, (src >> 15) as u8);
        self.write_rn_w(register, src << 1)?;

        Ok(self.calc_state(StateType::I, 1).await?)
    }

    pub(in super::super) async fn shal_l(&mut self, opcode: u16) -> Result<u8> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let src = self.read_rn_l(register)?;
        if src & 0x40000000 == 0x40000000 {
            self.write_ccr(CCR::N, 1)
        } else {
            self.write_ccr(CCR::N, 0);
        }
        if (src << 1) == 0 {
            self.write_ccr(CCR::Z, 1)
        } else {
            self.write_ccr(CCR::Z, 0);
        }
        if src & 0x80000000 == 0x80000000 {
            self.write_ccr(CCR::V, 1)
        } else {
            self.write_ccr(CCR::V, 0);
        }
        self.write_ccr(CCR::C, (src >> 31) as u8);
        self.write_rn_l(register, src << 1)?;

        Ok(self.calc_state(StateType::I, 1).await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::{cpu::Cpu, memory::MEMORY_START_ADDR};

    #[tokio::test]
    async fn test_shal_b() {
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00000111;

        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x10, 0x80]);
        cpu.write_rn_b(0, 0b0101_0101).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00001000);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0b1010_1010);

        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00000111;

        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x10, 0x8f]);
        cpu.write_rn_b(0xf, 0b0101_0101).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00001000);
        assert_eq!(cpu.read_rn_b(0xf).unwrap(), 0b1010_1010);

        // check V
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001100;

        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x10, 0x80]);
        cpu.write_rn_b(0, 0b1010_1010).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000011);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0b0101_0100);

        // check CCR C, Z
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001000;

        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x10, 0x80]);
        cpu.write_rn_b(0, 0b1000_0000).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000111);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0);
    }

    #[tokio::test]
    async fn test_shal_w() {
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00000111;

        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x10, 0x90]);
        cpu.write_rn_w(0, 0b0101_0101_0101_0101).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00001000);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0b1010_1010_1010_1010);

        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00000111;

        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x10, 0x9f]);
        cpu.write_rn_w(0xf, 0b0101_0101_0101_0101).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00001000);
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0b1010_1010_1010_1010);

        // check V
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001100;

        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x10, 0x90]);
        cpu.write_rn_w(0, 0b1010_1010_1010_1010).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000011);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0b0101_0101_0101_0100);

        // check CCR C, Z
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001000;

        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x10, 0x90]);
        cpu.write_rn_w(0, 0b1000_0000_0000_0000).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000111);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0);
    }

    #[tokio::test]
    async fn test_shal_l() {
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00000111;

        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x10, 0xb0]);
        cpu.write_rn_l(0, 0b0101_0101_0101_0101_0101_0101_0101_0101)
            .unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00001000);
        assert_eq!(
            cpu.read_rn_l(0).unwrap(),
            0b1010_1010_1010_1010_1010_1010_1010_1010
        );

        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00000111;

        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x10, 0xb7]);
        cpu.write_rn_l(7, 0b0101_0101_0101_0101_0101_0101_0101_0101)
            .unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00001000);
        assert_eq!(
            cpu.read_rn_l(7).unwrap(),
            0b1010_1010_1010_1010_1010_1010_1010_1010
        );

        // check V
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001100;

        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x10, 0xb0]);
        cpu.write_rn_l(0, 0b1010_1010_1010_1010_1010_1010_1010_1010)
            .unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000011);
        assert_eq!(
            cpu.read_rn_l(0).unwrap(),
            0b0101_0101_0101_0101_0101_0101_0101_0100
        );

        // check CCR C, Z
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001000;

        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x10, 0xb0]);
        cpu.write_rn_l(0, 0b1000_0000_0000_0000_0000_0000_0000_0000)
            .unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000111);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0);
    }
}
