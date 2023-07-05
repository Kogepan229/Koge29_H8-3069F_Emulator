use crate::cpu::Cpu;
use anyhow::{bail, Result};

impl Cpu {
    pub(in super::super) fn bset_rn_from_imm(&mut self, opcode: u16) -> Result<usize> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let value = self.read_rn_b(register)?;
        let imm = Cpu::get_nibble_opcode(opcode, 3)? & 7;
        self.write_rn_b(register, value | (1 << imm))?;
        Ok(2)
    }

    pub(in super::super) fn bset_rn_from_rn(&mut self, opcode: u16) -> Result<usize> {
        let register_bit = Cpu::get_nibble_opcode(opcode, 3)?;
        let register_value = Cpu::get_nibble_opcode(opcode, 4)?;
        let bit = self.read_rn_b(register_bit)? & 7;
        let value = self.read_rn_b(register_value)?;
        self.write_rn_b(register_value, value | (1 << bit))?;
        Ok(2)
    }

    pub(in super::super) async fn bset_ern(&mut self, opcode: u16, opcode2: u16) -> Result<usize> {
        match opcode2 & 0xff0f {
            0x7000 => {
                let register = Cpu::get_nibble_opcode(opcode, 3)?;
                let value = self.read_ern_b(register).await?;
                let imm = Cpu::get_nibble_opcode(opcode2, 3)? & 7;
                self.write_ern_b(register, value | (1 << imm)).await?;
            }
            0x6000 => {
                let register_bit = Cpu::get_nibble_opcode(opcode2, 3)?;
                let register_value = Cpu::get_nibble_opcode(opcode, 3)?;
                let bit = self.read_rn_b(register_bit)? & 7;
                let value = self.read_ern_b(register_value).await?;
                self.write_ern_b(register_value, value | (1 << bit)).await?;
            }
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
        return Ok(8);
    }

    pub(in super::super) async fn bset_abs(&mut self, opcode: u16, opcode2: u16) -> Result<usize> {
        match opcode2 & 0xff0f {
            0x7000 => {
                let imm = Cpu::get_nibble_opcode(opcode2, 3)? & 7;
                let value = self.read_abs8_b(opcode as u8).await?;
                self.write_abs8_b(opcode as u8, value | (1 << imm)).await?;
            }
            0x6000 => {
                let register = Cpu::get_nibble_opcode(opcode2, 3)?;
                let bit = self.read_rn_b(register)? & 7;
                let value = self.read_abs8_b(opcode as u8).await?;
                self.write_abs8_b(opcode as u8, value | (1 << bit)).await?;
            }
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
        return Ok(8);
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    #[tokio::test]
    async fn test_bset_rn_from_imm() {
        // bit 0
        let mut cpu = Cpu::new();
        cpu.write_rn_b(0, 0xfe).unwrap();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x70, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0xff);

        // bit 7
        let mut cpu = Cpu::new();
        cpu.write_rn_b(0, 0x7f).unwrap();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x70, 0x70]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0xff);

        // register 0xf
        let mut cpu = Cpu::new();
        cpu.write_rn_b(0xf, 0xfe).unwrap();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x70, 0x0f]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.read_rn_b(0xf).unwrap(), 0xff);
    }

    #[tokio::test]
    async fn test_bset_rn_from_rn() {
        // bit 0
        let mut cpu = Cpu::new();
        cpu.write_rn_b(0, 0).unwrap();
        cpu.write_rn_b(0xf, 0xfe).unwrap();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x60, 0x0f]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.read_rn_b(0xf).unwrap(), 0xff);

        // bit 7
        let mut cpu = Cpu::new();
        cpu.write_rn_b(0, 7).unwrap();
        cpu.write_rn_b(0xf, 0x7f).unwrap();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x60, 0x0f]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.read_rn_b(0xf).unwrap(), 0xff);

        // register 0xf
        let mut cpu = Cpu::new();
        cpu.write_rn_b(0xf, 0).unwrap();
        cpu.write_rn_b(0, 0xfe).unwrap();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x60, 0xf0]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0xff);
    }

    #[tokio::test]
    async fn test_bset_ern() {
        ////////
        // imm

        // bit 0
        let mut cpu = Cpu::new();
        cpu.write_abs24_b(0xffcf20, 0xfe).await.unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7d, 0x00, 0x70, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.read_ern_b(0).await.unwrap(), 0xff);

        // bit 7
        let mut cpu = Cpu::new();
        cpu.write_abs24_b(0xffcf20, 0x7f).await.unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7d, 0x00, 0x70, 0x70]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.read_ern_b(0).await.unwrap(), 0xff);

        // register 7
        let mut cpu = Cpu::new();
        cpu.write_abs24_b(0xffcf20, 0x7f).await.unwrap();
        cpu.write_rn_l(7, 0xffcf20).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7d, 0x70, 0x70, 0x70]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.read_ern_b(7).await.unwrap(), 0xff);

        ////////
        // rn

        // bit 0
        let mut cpu = Cpu::new();
        cpu.write_abs24_b(0xffcf20, 0xfe).await.unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.write_rn_b(0xf, 0).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7d, 0x00, 0x60, 0xf0]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.read_ern_b(0).await.unwrap(), 0xff);

        // bit 7
        let mut cpu = Cpu::new();
        cpu.write_abs24_b(0xffcf20, 0x7f).await.unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.write_rn_b(0xf, 7).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7d, 0x00, 0x60, 0xf0]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.read_ern_b(0).await.unwrap(), 0xff);

        // register 7
        let mut cpu = Cpu::new();
        cpu.write_abs24_b(0xffcf20, 0x7f).await.unwrap();
        cpu.write_rn_l(7, 0xffcf20).unwrap();
        cpu.write_rn_b(0, 7).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7d, 0x70, 0x60, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.read_ern_b(7).await.unwrap(), 0xff);
    }

    #[tokio::test]
    async fn test_bset_abs() {
        ////////
        // imm

        // bit 0
        let mut cpu = Cpu::new();
        cpu.write_abs24_b(0xffff12, 0xfe).await.unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7f, 0x12, 0x70, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.read_abs24_b(0xffff12).await.unwrap(), 0xff);

        // bit 7
        let mut cpu = Cpu::new();
        cpu.write_abs24_b(0xffff12, 0x7f).await.unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7f, 0x12, 0x70, 0x70]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.read_abs24_b(0xffff12).await.unwrap(), 0xff);

        ////////
        // rn

        // bit 0
        let mut cpu = Cpu::new();
        cpu.write_abs24_b(0xffff12, 0xfe).await.unwrap();
        cpu.write_rn_b(0, 0).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7f, 0x12, 0x60, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.read_abs24_b(0xffff12).await.unwrap(), 0xff);

        // bit 7
        let mut cpu = Cpu::new();
        cpu.write_abs24_b(0xffff12, 0x7f).await.unwrap();
        cpu.write_rn_b(0, 7).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7f, 0x12, 0x60, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.read_abs24_b(0xffff12).await.unwrap(), 0xff);

        // register 0xf
        let mut cpu = Cpu::new();
        cpu.write_abs24_b(0xffff12, 0x7f).await.unwrap();
        cpu.write_rn_b(0xf, 7).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7f, 0x12, 0x60, 0xf0]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.read_abs24_b(0xffff12).await.unwrap(), 0xff);
    }
}
