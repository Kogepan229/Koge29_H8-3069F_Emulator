use crate::cpu::{Cpu, CCR};
use anyhow::Result;

impl Cpu {
    pub(in super::super) fn extu_w(&mut self, opcode: u16) -> Result<usize> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let result = self.read_rn_w(register)? & 0x00ff;
        self.write_rn_w(register, result)?;

        self.write_ccr(CCR::N, 0);
        if result == 0 {
            self.write_ccr(CCR::Z, 1);
        } else {
            self.write_ccr(CCR::Z, 0);
        }
        self.write_ccr(CCR::V, 0);

        Ok(2)
    }

    pub(in super::super) fn extu_l(&mut self, opcode: u16) -> Result<usize> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let result = self.read_rn_l(register)? & 0x0000ffff;
        self.write_rn_l(register, result)?;

        self.write_ccr(CCR::N, 0);
        if result == 0 {
            self.write_ccr(CCR::Z, 1);
        } else {
            self.write_ccr(CCR::Z, 0);
        }
        self.write_ccr(CCR::V, 0);

        Ok(2)
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    #[tokio::test]
    async fn test_extu_w() {
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001110;
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x17, 0x50]);
        cpu.write_rn_w(0, 0xb6a5).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0xa5);

        // register 0xf
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001110;
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x17, 0x5f]);
        cpu.write_rn_w(0xf, 0xb6a5).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0xa5);

        // check CCR Z
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001110;
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x17, 0x50]);
        cpu.write_rn_w(0, 0xb600).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000100);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0);

        // check CCR expected unchanged
        let mut cpu = Cpu::new();
        cpu.ccr = 0b11111111;
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x17, 0x50]);
        cpu.write_rn_w(0, 0xb6a5).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b11110001);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0xa5);
    }

    #[tokio::test]
    async fn test_extu_l() {
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001110;
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x17, 0x70]);
        cpu.write_rn_l(0, 0xd8c7b6a5).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0x0000b6a5);

        // register 7
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001110;
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x17, 0x77]);
        cpu.write_rn_l(7, 0xd8c7b6a5).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(7).unwrap(), 0x0000b6a5);

        // check CCR Z
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001110;
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x17, 0x70]);
        cpu.write_rn_l(0, 0xd8c70000).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000100);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0);

        // check CCR expected unchanged
        let mut cpu = Cpu::new();
        cpu.ccr = 0b11111111;
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x17, 0x70]);
        cpu.write_rn_l(0, 0xd8c7b6a5).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b11110001);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0xb6a5);
    }
}
