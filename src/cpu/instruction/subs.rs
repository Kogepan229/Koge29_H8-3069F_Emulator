use crate::cpu::Cpu;
use anyhow::Result;

impl Cpu {
    pub(in super::super) fn subs1(&mut self, opcode: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            let register = Cpu::get_nibble_opcode(opcode, 4)?;
            let value = self.read_rn_l(register)?;
            self.write_rn_l(register, value.wrapping_add_signed(-1))?;
            Ok(2)
        };
        f()
    }

    pub(in super::super) fn subs2(&mut self, opcode: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            let register = Cpu::get_nibble_opcode(opcode, 4)?;
            let value = self.read_rn_l(register)?;
            self.write_rn_l(register, value.wrapping_add_signed(-2))?;
            Ok(2)
        };
        f()
    }

    pub(in super::super) fn subs4(&mut self, opcode: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            let register = Cpu::get_nibble_opcode(opcode, 4)?;
            let value = self.read_rn_l(register)?;
            self.write_rn_l(register, value.wrapping_add_signed(-4))?;
            Ok(2)
        };
        f()
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    #[tokio::test]
    async fn test_subs1() {
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x1b, 0x00]);
        cpu.write_rn_l(0, 10).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 9);

        // register 7
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x1b, 0x07]);
        cpu.write_rn_l(7, 10).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(7).unwrap(), 9);

        // overflow
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x1b, 0x00]);
        cpu.write_rn_l(0, std::i32::MIN as u32).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), std::i32::MAX as u32);

        // overflow2
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x1b, 0x00]);
        cpu.write_rn_l(0, std::u32::MIN).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), std::u32::MAX);

        // u32 MAX - 1
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x1b, 0x00]);
        cpu.write_rn_l(0, std::u32::MAX).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), std::u32::MAX - 1);

        // 0 - 1
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x1b, 0x00]);
        cpu.write_rn_l(0, 0).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), -1i32 as u32);
    }

    #[tokio::test]
    async fn test_subs2() {
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x1b, 0x80]);
        cpu.write_rn_l(0, 10).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 8);

        // register 7
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x1b, 0x87]);
        cpu.write_rn_l(7, 10).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(7).unwrap(), 8);

        // overflow
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x1b, 0x80]);
        cpu.write_rn_l(0, std::i32::MIN as u32).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), std::i32::MAX as u32 - 1);

        // u32 MAX - 1
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x1b, 0x80]);
        cpu.write_rn_l(0, std::u32::MAX).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), std::u32::MAX - 2);

        // 0 - 1
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x1b, 0x80]);
        cpu.write_rn_l(0, 0).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), -2i32 as u32);
    }

    #[tokio::test]
    async fn test_subs4() {
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x1b, 0x90]);
        cpu.write_rn_l(0, 10).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 6);

        // register 7
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x1b, 0x97]);
        cpu.write_rn_l(7, 10).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(7).unwrap(), 6);

        // overflow
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x1b, 0x90]);
        cpu.write_rn_l(0, std::i32::MIN as u32).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), std::i32::MAX as u32 - 3);

        // u32 MAX - 1
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x1b, 0x90]);
        cpu.write_rn_l(0, std::u32::MAX).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), std::u32::MAX - 4);

        // 0 - 1
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x1b, 0x90]);
        cpu.write_rn_l(0, 0).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), -4i32 as u32);
    }
}
