use crate::cpu::Cpu;
use anyhow::{Context as _, Result};

impl<'a> Cpu<'a> {
    pub(in super::super) fn write_ern_b(&mut self, register_field: u8, value: u8) -> Result<()> {
        let addr = self
            .read_rn_l(register_field)
            .with_context(|| format!("register [{:x}] value [{:x}]", register_field, value))?;
        self.write_abs24_b(addr & 0x00ffffff, value)
            .with_context(|| format!("register [{:x}] value [{:x}]", register_field, value))?;
        Ok(())
    }

    pub(in super::super) fn read_ern_b(&self, register_field: u8) -> Result<u8> {
        let addr = self
            .read_rn_l(register_field)
            .with_context(|| format!("register [{:x}]", register_field))?;
        self.read_abs24_b(addr & 0x00ffffff)
            .with_context(|| format!("register [{:x}]", register_field))
    }

    pub(in super::super) fn write_ern_w(&mut self, register_field: u8, value: u16) -> Result<()> {
        let addr = self
            .read_rn_l(register_field)
            .with_context(|| format!("register [{:x}] value [{:x}]", register_field, value))?;
        self.write_abs24_w(addr & 0x00ffffff, value)
            .with_context(|| format!("register [{:x}] value [{:x}]", register_field, value))?;
        Ok(())
    }

    pub(in super::super) fn read_ern_w(&self, register_field: u8) -> Result<u16> {
        let addr = self
            .read_rn_l(register_field)
            .with_context(|| format!("register [{:x}]", register_field))?;
        self.read_abs24_w(addr & 0x00ffffff)
            .with_context(|| format!("register [{:x}]", register_field))
    }

    pub(in super::super) fn write_ern_l(&mut self, register_field: u8, value: u32) -> Result<()> {
        let addr = self
            .read_rn_l(register_field)
            .with_context(|| format!("register [{:x}] value [{:x}]", register_field, value))?;
        self.write_abs24_l(addr & 0x00ffffff, value)
            .with_context(|| format!("register [{:x}] value [{:x}]", register_field, value))?;
        Ok(())
    }

    pub(in super::super) fn read_ern_l(&self, register_field: u8) -> Result<u32> {
        let addr = self
            .read_rn_l(register_field)
            .with_context(|| format!("register [{:x}]", register_field))?;
        self.read_abs24_l(addr & 0x00ffffff)
            .with_context(|| format!("register [{:x}]", register_field))
    }
}

#[cfg(test)]
mod tests {
    use crate::{cpu::Cpu, mcu::Mcu};
    #[test]
    fn test_write_ern_b() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.er[0] = 0xffff10;
        cpu.write_ern_b(0, 0xff).unwrap();

        assert_eq!(cpu.read_abs24_b(0xffff10).unwrap(), 0xff)
    }

    #[test]
    fn test_read_ern_b() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.er[0] = 0xffff10;
        cpu.write_abs24_b(0xffff10, 0xff).unwrap();
        assert_eq!(cpu.read_ern_b(0).unwrap(), 0xff)
    }

    #[test]
    fn test_write_ern_w() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.er[0] = 0xffff10;
        cpu.write_ern_w(0, 0x0fff).unwrap();

        assert_eq!(cpu.read_abs24_w(0xffff10).unwrap(), 0x0fff)
    }

    #[test]
    fn test_read_ern_w() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.er[0] = 0xffff10;
        cpu.write_abs24_w(0xffff10, 0x0fff).unwrap();
        assert_eq!(cpu.read_ern_w(0).unwrap(), 0x0fff)
    }

    #[test]
    fn test_write_ern_l() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.er[0] = 0xffff10;
        cpu.write_ern_l(0, 0x0f0fff0f).unwrap();

        assert_eq!(cpu.read_abs24_l(0xffff10).unwrap(), 0x0f0fff0f)
    }

    #[test]
    fn test_read_ern_l() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.er[0] = 0xffff10;
        cpu.write_abs24_l(0xffff10, 0x0f0fff0f).unwrap();
        assert_eq!(cpu.read_ern_l(0).unwrap(), 0x0f0fff0f)
    }
}
