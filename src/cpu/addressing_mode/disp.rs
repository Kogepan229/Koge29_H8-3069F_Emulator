use crate::cpu::{Cpu, ADDRESS_MASK};
use anyhow::{anyhow, Result};

impl Cpu {
    pub(in super::super) fn get_addr_disp16(&self, register_field: u8, disp: u16) -> Result<u32> {
        let addr = self.read_rn_l(register_field)?;
        let result = addr
            .checked_add_signed((disp as i16) as i32)
            .ok_or_else(|| anyhow!("attempt to add with overflow [{:x} + {:x}]", addr, disp as i16))?
            & ADDRESS_MASK;
        Ok(result)
    }

    pub(in super::super) fn get_addr_disp24(&self, register_field: u8, disp: u32) -> Result<u32> {
        let addr = self.read_rn_l(register_field)?;
        if disp & 0x800000 == 0x000000 {
            Ok((addr + disp) & ADDRESS_MASK)
        } else {
            let result = addr
                .checked_add_signed((0xff000000 + disp) as i32)
                .ok_or_else(|| anyhow!("attempt to add with overflow [{:x} + {:x}]", addr, (0xffff0000 + disp)))?
                & ADDRESS_MASK;
            Ok(result)
        }
    }

    pub(in super::super) fn write_disp16_ern_b(&mut self, register_field: u8, disp: u16, value: u8) -> Result<()> {
        self.write_abs24_b(self.get_addr_disp16(register_field, disp)?, value)?;
        Ok(())
    }

    pub(in super::super) fn read_disp16_ern_b(&self, register_field: u8, disp: u16) -> Result<u8> {
        Ok(self.read_abs24_b(self.get_addr_disp16(register_field, disp)?)?)
    }

    pub(in super::super) fn write_disp16_ern_w(&mut self, register_field: u8, disp: u16, value: u16) -> Result<()> {
        self.write_abs24_w(self.get_addr_disp16(register_field, disp)?, value)?;
        Ok(())
    }

    pub(in super::super) fn read_disp16_ern_w(&self, register_field: u8, disp: u16) -> Result<u16> {
        Ok(self.read_abs24_w(self.get_addr_disp16(register_field, disp)?)?)
    }

    pub(in super::super) fn write_disp16_ern_l(&mut self, register_field: u8, disp: u16, value: u32) -> Result<()> {
        self.write_abs24_l(self.get_addr_disp16(register_field, disp)?, value)?;
        Ok(())
    }

    pub(in super::super) fn read_disp16_ern_l(&self, register_field: u8, disp: u16) -> Result<u32> {
        Ok(self.read_abs24_l(self.get_addr_disp16(register_field, disp)?)?)
    }

    ////
    pub(in super::super) fn write_disp24_ern_b(&mut self, register_field: u8, disp: u32, value: u8) -> Result<()> {
        self.write_abs24_b(self.get_addr_disp24(register_field, disp)?, value)?;
        Ok(())
    }

    pub(in super::super) fn read_disp24_ern_b(&self, register_field: u8, disp: u32) -> Result<u8> {
        Ok(self.read_abs24_b(self.get_addr_disp24(register_field, disp)?)?)
    }

    pub(in super::super) fn write_disp24_ern_w(&mut self, register_field: u8, disp: u32, value: u16) -> Result<()> {
        self.write_abs24_w(self.get_addr_disp24(register_field, disp)?, value)?;
        Ok(())
    }

    pub(in super::super) fn read_disp24_ern_w(&self, register_field: u8, disp: u32) -> Result<u16> {
        Ok(self.read_abs24_w(self.get_addr_disp24(register_field, disp)?)?)
    }

    pub(in super::super) fn write_disp24_ern_l(&mut self, register_field: u8, disp: u32, value: u32) -> Result<()> {
        self.write_abs24_l(self.get_addr_disp24(register_field, disp)?, value)?;
        Ok(())
    }

    pub(in super::super) fn read_disp24_ern_l(&self, register_field: u8, disp: u32) -> Result<u32> {
        Ok(self.read_abs24_l(self.get_addr_disp24(register_field, disp)?)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    #[test]
    fn test_write_disp16_ern_b() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xfff000;
        cpu.write_disp16_ern_b(0, 0x0f10, 0xff).unwrap();
        assert_eq!(cpu.read_abs24_b(0xffff10).unwrap(), 0xff);

        cpu.er[1] = 0xfffe0a;
        cpu.write_disp16_ern_b(1, 0xfff6, 0xff).unwrap();
        assert_eq!(cpu.read_abs24_b(0xfffe00).unwrap(), 0xff);
    }

    #[test]
    fn test_read_disp16_ern_b() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xfff000;
        cpu.write_abs24_b(0xffff10, 0xff).unwrap();
        assert_eq!(cpu.read_disp16_ern_b(0, 0x0f10).unwrap(), 0xff);

        cpu.er[1] = 0xfffe0a;
        cpu.write_abs24_b(0xfffe00, 0xff).unwrap();
        assert_eq!(cpu.read_disp16_ern_b(1, 0xfff6).unwrap(), 0xff);
    }

    #[test]
    fn test_write_disp16_ern_w() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xfff000;
        cpu.write_disp16_ern_w(0, 0x0f10, 0x0fff).unwrap();
        assert_eq!(cpu.read_abs24_w(0xffff10).unwrap(), 0x0fff);

        cpu.er[1] = 0xfffe0a;
        cpu.write_disp16_ern_w(1, 0xfff6, 0x0fff).unwrap();
        assert_eq!(cpu.read_abs24_w(0xfffe00).unwrap(), 0x0fff);
    }

    #[test]
    fn test_read_disp16_ern_w() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xfff000;
        cpu.write_abs24_w(0xffff10, 0x0fff).unwrap();
        assert_eq!(cpu.read_disp16_ern_w(0, 0x0f10).unwrap(), 0x0fff);

        cpu.er[1] = 0xfffe0a;
        cpu.write_abs24_w(0xfffe00, 0x0fff).unwrap();
        assert_eq!(cpu.read_disp16_ern_w(1, 0xfff6).unwrap(), 0x0fff);
    }

    #[test]
    fn test_write_disp16_ern_l() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xfff000;
        cpu.write_disp16_ern_l(0, 0x0f10, 0x0f0fff0f).unwrap();
        assert_eq!(cpu.read_abs24_l(0xffff10).unwrap(), 0x0f0fff0f);

        cpu.er[1] = 0xfffe0a;
        cpu.write_disp16_ern_l(1, 0xfff6, 0x0f0fff0f).unwrap();
        assert_eq!(cpu.read_abs24_l(0xfffe00).unwrap(), 0x0f0fff0f);
    }

    #[test]
    fn test_read_disp16_ern_l() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xfff000;
        cpu.write_abs24_l(0xffff10, 0x0f0fff0f).unwrap();
        assert_eq!(cpu.read_disp16_ern_l(0, 0x0f10).unwrap(), 0x0f0fff0f);

        cpu.er[1] = 0xfffe0a;
        cpu.write_abs24_l(0xfffe00, 0x0f0fff0f).unwrap();
        assert_eq!(cpu.read_disp16_ern_l(1, 0xfff6).unwrap(), 0x0f0fff0f);
    }

    #[test]
    fn test_write_disp24_ern_b() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xfff000;
        cpu.write_disp24_ern_b(0, 0x000f10, 0xff).unwrap();
        assert_eq!(cpu.read_abs24_b(0xffff10).unwrap(), 0xff);

        cpu.er[1] = 0xfffe0a;
        cpu.write_disp24_ern_b(1, 0xfffff6, 0xff).unwrap();
        assert_eq!(cpu.read_abs24_b(0xfffe00).unwrap(), 0xff);
    }

    #[test]
    fn test_read_disp24_ern_b() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xfff000;
        cpu.write_abs24_b(0xffff10, 0xff).unwrap();
        assert_eq!(cpu.read_disp24_ern_b(0, 0x000f10).unwrap(), 0xff);

        cpu.er[1] = 0xfffe0a;
        cpu.write_abs24_b(0xfffe00, 0xff).unwrap();
        assert_eq!(cpu.read_disp24_ern_b(1, 0xfffff6).unwrap(), 0xff);
    }

    #[test]
    fn test_write_disp24_ern_w() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xfff000;
        cpu.write_disp24_ern_w(0, 0x000f10, 0x0fff).unwrap();
        assert_eq!(cpu.read_abs24_w(0xffff10).unwrap(), 0x0fff);

        cpu.er[1] = 0xfffe0a;
        cpu.write_disp24_ern_w(1, 0xfffff6, 0x0fff).unwrap();
        assert_eq!(cpu.read_abs24_w(0xfffe00).unwrap(), 0x0fff);
    }

    #[test]
    fn test_read_disp24_ern_w() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xfff000;
        cpu.write_abs24_w(0xffff10, 0x0fff).unwrap();
        assert_eq!(cpu.read_disp24_ern_w(0, 0x000f10).unwrap(), 0x0fff);

        cpu.er[1] = 0xfffe0a;
        cpu.write_abs24_w(0xfffe00, 0x0fff).unwrap();
        assert_eq!(cpu.read_disp24_ern_w(1, 0xfffff6).unwrap(), 0x0fff);
    }

    #[test]
    fn test_write_disp24_ern_l() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xfff000;
        cpu.write_disp24_ern_l(0, 0x000f10, 0x0f0fff0f).unwrap();
        assert_eq!(cpu.read_abs24_l(0xffff10).unwrap(), 0x0f0fff0f);

        cpu.er[1] = 0xfffe0a;
        cpu.write_disp24_ern_l(1, 0xfffff6, 0x0f0fff0f).unwrap();
        assert_eq!(cpu.read_abs24_l(0xfffe00).unwrap(), 0x0f0fff0f);
    }

    #[test]
    fn test_read_disp24_ern_l() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xfff000;
        cpu.write_abs24_l(0xffff10, 0x0f0fff0f).unwrap();
        assert_eq!(cpu.read_disp24_ern_l(0, 0x000f10).unwrap(), 0x0f0fff0f);

        cpu.er[1] = 0xfffe0a;
        cpu.write_abs24_l(0xfffe00, 0x0f0fff0f).unwrap();
        assert_eq!(cpu.read_disp24_ern_l(1, 0xfffff6).unwrap(), 0x0f0fff0f);
    }
}
