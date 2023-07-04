use crate::cpu::Cpu;
use anyhow::Result;

impl Cpu {
    pub(in super::super) fn adds1(&mut self, opcode: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            let register = Cpu::get_nibble_opcode(opcode, 4)?;
            let value = self.read_rn_l(register)?;
            self.write_rn_l(register, value.wrapping_add(1))?;
            Ok(2)
        };
        f()
    }
    pub(in super::super) fn adds2(&mut self, opcode: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            let register = Cpu::get_nibble_opcode(opcode, 4)?;
            let value = self.read_rn_l(register)?;
            self.write_rn_l(register, value.wrapping_add(2))?;
            Ok(2)
        };
        f()
    }
    pub(in super::super) fn adds4(&mut self, opcode: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            let register = Cpu::get_nibble_opcode(opcode, 4)?;
            let value = self.read_rn_l(register)?;
            self.write_rn_l(register, value.wrapping_add(4))?;
            Ok(2)
        };
        f()
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    #[tokio::test]
    async fn test_adds1() {
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x0b, 0x00]);
        cpu.write_rn_l(0, 10).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 11);

        // register 7
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x0b, 0x07]);
        cpu.write_rn_l(7, 10).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(7).unwrap(), 11);

        // overflow
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x0b, 0x00]);
        cpu.write_rn_l(0, std::u32::MAX).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), std::u32::MIN);

        // u32 MIN + 1
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x0b, 0x00]);
        cpu.write_rn_l(0, std::u32::MIN).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), std::u32::MIN + 1);

        // 0 + 1
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x0b, 0x00]);
        cpu.write_rn_l(0, 0).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 1);

        // -1 + 1
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x0b, 0x00]);
        cpu.write_rn_l(0, -1i32 as u32).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0);
    }

    #[tokio::test]
    async fn test_adds2() {
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x0b, 0x80]);
        cpu.write_rn_l(0, 10).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 12);

        // register 7
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x0b, 0x87]);
        cpu.write_rn_l(7, 10).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(7).unwrap(), 12);

        // overflow
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x0b, 0x80]);
        cpu.write_rn_l(0, std::u32::MAX).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), std::u32::MIN + 1);

        // u32 MIN + 1
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x0b, 0x80]);
        cpu.write_rn_l(0, std::u32::MIN).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), std::u32::MIN + 2);

        // 0 + 1
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x0b, 0x80]);
        cpu.write_rn_l(0, 0).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 2);

        // -1 + 1
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x0b, 0x80]);
        cpu.write_rn_l(0, -2i32 as u32).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0);
    }

    #[tokio::test]
    async fn test_adds4() {
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x0b, 0x90]);
        cpu.write_rn_l(0, 10).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 14);

        // register 7
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x0b, 0x97]);
        cpu.write_rn_l(7, 10).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(7).unwrap(), 14);

        // overflow
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x0b, 0x90]);
        cpu.write_rn_l(0, std::u32::MAX).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), std::u32::MIN + 3);

        // u32 MIN + 1
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x0b, 0x90]);
        cpu.write_rn_l(0, std::u32::MIN).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), std::u32::MIN + 4);

        // 0 + 1
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x0b, 0x90]);
        cpu.write_rn_l(0, 0).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 4);

        // -1 + 1
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x0b, 0x90]);
        cpu.write_rn_l(0, -4i32 as u32).unwrap();
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0);
    }
}
