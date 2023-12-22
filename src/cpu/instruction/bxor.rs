use crate::cpu::{Cpu, StateType, CCR};
use anyhow::Result;

impl Cpu {
    pub(in super::super) async fn bxor_rn(&mut self, opcode: u16) -> Result<u8> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let value = self.read_rn_b(register)?;
        let imm = Cpu::get_nibble_opcode(opcode, 3)? & 7;
        self.write_ccr(CCR::C, ((value >> imm) & 1) ^ self.read_ccr(CCR::C));
        Ok(self.calc_state(StateType::I, 1).await?)
    }

    pub(in super::super) async fn bxor_ern(&mut self, opcode: u16, opcode2: u16) -> Result<u8> {
        let register = Cpu::get_nibble_opcode(opcode, 3)?;
        let access_addr = self.get_addr_ern(register)?;
        let value = self.read_ern_b(register).await?;
        let imm = Cpu::get_nibble_opcode(opcode2, 3)? & 7;
        self.write_ccr(CCR::C, ((value >> imm) & 1) ^ self.read_ccr(CCR::C));
        Ok(self.calc_state(StateType::I, 2).await?
            + self
                .calc_state_with_addr(StateType::L, 1, access_addr)
                .await?)
    }

    pub(in super::super) async fn bxor_abs(&mut self, opcode: u16, opcode2: u16) -> Result<u8> {
        let imm = Cpu::get_nibble_opcode(opcode2, 3)? & 7;
        let value = self.read_abs8_b(opcode as u8).await?;
        self.write_ccr(CCR::C, ((value >> imm) & 1) ^ self.read_ccr(CCR::C));
        let access_addr = self.get_addr_abs8(opcode as u8);
        Ok(self.calc_state(StateType::I, 2).await?
            + self
                .calc_state_with_addr(StateType::L, 1, access_addr)
                .await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    #[tokio::test]
    async fn test_bxor_rn() {
        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_rn_b(0, 0x01).unwrap();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x75, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);

        // bit7
        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_rn_b(0, 0x80).unwrap();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x75, 0x70]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);

        // register 0xf
        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_rn_b(0xf, 0x01).unwrap();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x75, 0x0f]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);

        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_rn_b(0, 0xfe).unwrap();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x75, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 1);

        let mut cpu = Cpu::new();
        cpu.ccr = 0;
        cpu.write_rn_b(0, 0x01).unwrap();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x75, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 1);

        let mut cpu = Cpu::new();
        cpu.ccr = 0;
        cpu.write_rn_b(0, 0).unwrap();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x75, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
    }

    #[tokio::test]
    async fn test_bxor_ern() {
        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_abs24_b(0xffcf20, 0x01).await.unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7c, 0x00, 0x75, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 0);

        // bit 7
        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_abs24_b(0xffcf20, 0x80).await.unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7c, 0x00, 0x75, 0x70]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 0);

        // register 7
        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_abs24_b(0xffcf20, 0x01).await.unwrap();
        cpu.write_rn_l(7, 0xffcf20).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7c, 0x70, 0x75, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 0);

        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_abs24_b(0xffcf20, 0xfe).await.unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7c, 0x00, 0x75, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 1);

        let mut cpu = Cpu::new();
        cpu.ccr = 0;
        cpu.write_abs24_b(0xffcf20, 0x01).await.unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7c, 0x00, 0x75, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 1);

        let mut cpu = Cpu::new();
        cpu.ccr = 0;
        cpu.write_abs24_b(0xffcf20, 0).await.unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7c, 0x00, 0x75, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 0);
    }

    #[tokio::test]
    async fn test_bxor_abs() {
        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_abs24_b(0xffff12, 0x01).await.unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7e, 0x12, 0x75, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 0);

        // bit 7
        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_abs24_b(0xffff12, 0x80).await.unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7e, 0x12, 0x75, 0x70]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 0);

        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_abs24_b(0xffff12, 0xfe).await.unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7e, 0x12, 0x75, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 1);

        let mut cpu = Cpu::new();
        cpu.ccr = 0;
        cpu.write_abs24_b(0xffff12, 0x01).await.unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7e, 0x12, 0x75, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 1);

        let mut cpu = Cpu::new();
        cpu.ccr = 0;
        cpu.write_abs24_b(0xffff12, 0).await.unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7e, 0x12, 0x75, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 0);
    }
}
