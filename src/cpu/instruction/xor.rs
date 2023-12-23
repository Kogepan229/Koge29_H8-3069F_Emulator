use crate::cpu::{Cpu, StateType, CCR};
use anyhow::{Context as _, Result};

impl Cpu {
    pub(in super::super) async fn xor_b_imm(&mut self, opcode: u16) -> Result<u8> {
        let register = Cpu::get_nibble_opcode(opcode, 2)?;
        let result = self.read_rn_b(register)? ^ opcode as u8;
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

    pub(in super::super) async fn xor_b_rn(&mut self, opcode: u16) -> Result<u8> {
        let register_src = Cpu::get_nibble_opcode(opcode, 3)?;
        let register_dest = Cpu::get_nibble_opcode(opcode, 4)?;
        let result = self.read_rn_b(register_src)? ^ self.read_rn_b(register_dest)?;
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

    pub(in super::super) async fn xor_w_imm(&mut self, opcode: u16) -> Result<u8> {
        let opcode2 = self.fetch().await;

        let mut f = || -> Result<()> {
            let register = Cpu::get_nibble_opcode(opcode, 4)?;
            let result = self.read_rn_w(register)? ^ opcode2;
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

    pub(in super::super) async fn xor_w_rn(&mut self, opcode: u16) -> Result<u8> {
        let register_src = Cpu::get_nibble_opcode(opcode, 3)?;
        let register_dest = Cpu::get_nibble_opcode(opcode, 4)?;
        let result = self.read_rn_w(register_src)? ^ self.read_rn_w(register_dest)?;
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

    pub(in super::super) async fn xor_l_imm(&mut self, opcode: u16) -> Result<u8> {
        let imm = ((self.fetch().await as u32) << 16) | self.fetch().await as u32;

        let mut f = || -> Result<()> {
            let register = Cpu::get_nibble_opcode(opcode, 4)?;
            let result = self.read_rn_l(register)? ^ imm;
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

    pub(in super::super) async fn xor_l_rn(&mut self, _opcode: u16, opcode2: u16) -> Result<u8> {
        let mut f = || -> Result<()> {
            let register_src = Cpu::get_nibble_opcode(opcode2, 3)?;
            let register_dest = Cpu::get_nibble_opcode(opcode2, 4)?;
            let result = self.read_rn_l(register_src)? ^ self.read_rn_l(register_dest)?;
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
    async fn test_xor_b_imm() {
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00000111;
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0xd0, 0x7a]);
        cpu.write_rn_b(0, 0xaf).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0xd5);

        let mut cpu = Cpu::new();
        cpu.ccr = 0b00000111;
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0xdf, 0x7a]);
        cpu.write_rn_b(0xf, 0xaf).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_b(0xf).unwrap(), 0xd5);

        // check zero
        let mut cpu = Cpu::new();
        cpu.ccr = 0b000001011;
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0xd0, 0x55]);
        cpu.write_rn_b(0, 0x55).unwrap();
        let opcode = cpu.fetch().await;
        cpu.exec(opcode).await.unwrap();
        assert_eq!(cpu.ccr & 0b00001110, 0b00000100);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0);
    }

    #[tokio::test]
    async fn test_xor_b_rn() {
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00000111;
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x15, 0x0f]);
        cpu.write_rn_b(0, 0xaf).unwrap();
        cpu.write_rn_b(0xf, 0x7a).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        // check result
        assert_eq!(cpu.read_rn_b(0xf).unwrap(), 0xd5);
        // check no change
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0xaf);

        let mut cpu = Cpu::new();
        cpu.ccr = 0b00000111;
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x15, 0xf0]);
        cpu.write_rn_b(0xf, 0xaf).unwrap();
        cpu.write_rn_b(0, 0x7a).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        // check result
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0xd5);
        // check no change
        assert_eq!(cpu.read_rn_b(0xf).unwrap(), 0xaf);

        // check zero
        let mut cpu = Cpu::new();
        cpu.ccr = 0b000001011;
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x15, 0x0f]);
        cpu.write_rn_b(0xf, 0x55).unwrap();
        cpu.write_rn_b(0, 0x55).unwrap();
        let opcode = cpu.fetch().await;
        cpu.exec(opcode).await.unwrap();
        assert_eq!(cpu.ccr & 0b00001110, 0b00000100);
        assert_eq!(cpu.read_rn_b(0xf).unwrap(), 0);
    }

    #[tokio::test]
    async fn test_xor_w_imm() {
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00000111;
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x79, 0x50, 0x7a, 0x69]);
        cpu.write_rn_w(0, 0xaf9e).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 4);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0xd5f7);

        let mut cpu = Cpu::new();
        cpu.ccr = 0b00000111;
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x79, 0x5f, 0x7a, 0x69]);
        cpu.write_rn_w(0xf, 0xaf9e).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 4);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0xd5f7);

        // check zero
        let mut cpu = Cpu::new();
        cpu.ccr = 0b000001011;
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x79, 0x50, 0x55, 0x66]);
        cpu.write_rn_w(0, 0x5566).unwrap();
        let opcode = cpu.fetch().await;
        cpu.exec(opcode).await.unwrap();
        assert_eq!(cpu.ccr & 0b00001110, 0b00000100);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0);
    }

    #[tokio::test]
    async fn test_xor_w_rn() {
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00000111;
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x65, 0x0f]);
        cpu.write_rn_w(0, 0x7a69).unwrap();
        cpu.write_rn_w(0xf, 0xaf9e).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        // check result
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0xd5f7);
        // check no change
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0x7a69);

        let mut cpu = Cpu::new();
        cpu.ccr = 0b00000111;
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x65, 0xf0]);
        cpu.write_rn_w(0xf, 0x7a69).unwrap();
        cpu.write_rn_w(0, 0xaf9e).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        // check result
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0xd5f7);
        // check no change
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0x7a69);

        // check zero
        let mut cpu = Cpu::new();
        cpu.ccr = 0b000001011;
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x65, 0x0f]);
        cpu.write_rn_w(0xf, 0x5566).unwrap();
        cpu.write_rn_w(0, 0x5566).unwrap();
        let opcode = cpu.fetch().await;
        cpu.exec(opcode).await.unwrap();
        assert_eq!(cpu.ccr & 0b00001110, 0b00000100);
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0);
    }

    #[tokio::test]
    async fn test_xor_l_imm() {
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00000111;
        cpu.bus.lock().await.memory[0..6].copy_from_slice(&[0x7a, 0x50, 0x7a, 0x69, 0x58, 0x47]);
        cpu.write_rn_l(0, 0xaf9e8d7c).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0xd5f7d53b);

        let mut cpu = Cpu::new();
        cpu.ccr = 0b00000111;
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.lock().await.memory[0..6].copy_from_slice(&[0x7a, 0x57, 0x7a, 0x69, 0x58, 0x47]);
        cpu.write_rn_l(0x7, 0xaf9e8d7c).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_l(0x7).unwrap(), 0xd5f7d53b);

        // check zero
        let mut cpu = Cpu::new();
        cpu.ccr = 0b000001011;
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.lock().await.memory[0..6].copy_from_slice(&[0x7a, 0x50, 0x55, 0x66, 0x77, 0x88]);
        cpu.write_rn_l(0, 0x55667788).unwrap();
        let opcode = cpu.fetch().await;
        cpu.exec(opcode).await.unwrap();
        assert_eq!(cpu.ccr & 0b00001110, 0b00000100);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0);
    }

    #[tokio::test]
    async fn test_xor_l_rn() {
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00000111;
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x01, 0xf0, 0x65, 0x07]);
        cpu.write_rn_l(0, 0x7a695847).unwrap();
        cpu.write_rn_l(7, 0xaf9e8d7c).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 4);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        // check result
        assert_eq!(cpu.read_rn_l(7).unwrap(), 0xd5f7d53b);
        // check no change
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0x7a695847);

        let mut cpu = Cpu::new();
        cpu.ccr = 0b00000111;
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x01, 0xf0, 0x65, 0x70]);
        cpu.write_rn_l(7, 0x7a695847).unwrap();
        cpu.write_rn_l(0, 0xaf9e8d7c).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 4);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        // check result
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0xd5f7d53b);
        // check no change
        assert_eq!(cpu.read_rn_l(7).unwrap(), 0x7a695847);

        // check zero
        let mut cpu = Cpu::new();
        cpu.ccr = 0b000001011;
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x01, 0xf0, 0x65, 0x70]);
        cpu.write_rn_l(7, 0x55667788).unwrap();
        cpu.write_rn_l(0, 0x55667788).unwrap();
        let opcode = cpu.fetch().await;
        cpu.exec(opcode).await.unwrap();
        assert_eq!(cpu.ccr & 0b00001110, 0b00000100);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0);
    }
}
