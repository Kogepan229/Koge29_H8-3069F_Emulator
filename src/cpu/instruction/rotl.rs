use crate::cpu::{Cpu, StateType, CCR};
use anyhow::Result;

impl Cpu {
    pub(in super::super) async fn rotl_b(&mut self, opcode: u16) -> Result<u8> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let src = self.read_rn_b(register)?;
        let result = (src << 1) | (src >> 7);
        self.write_rn_b(register, result)?;
        if result & 0x80 == 0x80 {
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
        self.write_ccr(CCR::C, result & 1);

        Ok(self.calc_state(StateType::I, 1).await?)
    }

    pub(in super::super) async fn rotl_w(&mut self, opcode: u16) -> Result<u8> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let src = self.read_rn_w(register)?;
        let result = (src << 1) | (src >> 15);
        self.write_rn_w(register, result)?;
        if result & 0x8000 == 0x8000 {
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
        self.write_ccr(CCR::C, (result & 1) as u8);

        Ok(self.calc_state(StateType::I, 1).await?)
    }

    pub(in super::super) async fn rotl_l(&mut self, opcode: u16) -> Result<u8> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let src = self.read_rn_l(register)?;
        let result = (src << 1) | (src >> 31);
        self.write_rn_l(register, result)?;
        if result & 0x8000_0000 == 0x8000_0000 {
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
        self.write_ccr(CCR::C, (result & 1) as u8);

        Ok(self.calc_state(StateType::I, 1).await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    #[tokio::test]
    async fn test_rotl_b() {
        // check CCR N, V
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001111;

        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x12, 0x80]);
        cpu.write_rn_b(0, 0b0101_0101).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00001000);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0b1010_1010);

        // check register 0xf
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001111;

        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x12, 0x8f]);
        cpu.write_rn_b(0xf, 0b0101_0101).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00001000);
        assert_eq!(cpu.read_rn_b(0xf).unwrap(), 0b1010_1010);

        // check CCR Z
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001011;

        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x12, 0x80]);
        cpu.write_rn_b(0, 0).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000100);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0);

        // check CCR C, velue
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001110;

        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x12, 0x80]);
        cpu.write_rn_b(0, 0b1010_1010).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000001);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0b0101_0101);
    }

    #[tokio::test]
    async fn test_rotl_w() {
        // check CCR N, V
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001111;

        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x12, 0x90]);
        cpu.write_rn_w(0, 0b0101_0101_0101_0101).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00001000);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0b1010_1010_1010_1010);

        // check register 0xf
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001111;

        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x12, 0x9f]);
        cpu.write_rn_w(0xf, 0b0101_0101_0101_0101).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00001000);
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0b1010_1010_1010_1010);

        // check CCR Z
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001011;

        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x12, 0x90]);
        cpu.write_rn_w(0, 0).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000100);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0);

        // check CCR C, velue
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001110;

        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x12, 0x90]);
        cpu.write_rn_w(0, 0b1010_1010_1010_1010).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000001);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0b0101_0101_0101_0101);
    }

    #[tokio::test]
    async fn test_rotl_l() {
        // check CCR N, V
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001111;

        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x12, 0xb0]);
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

        // check register 7
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001111;

        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x12, 0xb7]);
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

        // check CCR Z
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001011;

        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x12, 0xb0]);
        cpu.write_rn_l(0, 0).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000100);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0);

        // check CCR C, velue
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001110;

        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x12, 0xb0]);
        cpu.write_rn_l(0, 0b1010_1010_1010_1010_1010_1010_1010_1010)
            .unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000001);
        assert_eq!(
            cpu.read_rn_l(0).unwrap(),
            0b0101_0101_0101_0101_0101_0101_0101_0101
        );
    }
}
