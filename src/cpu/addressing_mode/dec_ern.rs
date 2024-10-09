use crate::cpu::{Cpu, ADDRESS_MASK};
use anyhow::Result;

impl Cpu {
    pub(in super::super) fn write_dec_ern_b(&mut self, register_field: u8, value: u8) -> Result<()> {
        let addr = self.read_rn_l(register_field)?;
        self.write_abs24_b((addr - 1) & ADDRESS_MASK, value)?;
        self.write_rn_l(register_field, addr - 1)?;
        Ok(())
    }

    pub(in super::super) fn read_dec_ern_b(&mut self, register_field: u8) -> Result<u8> {
        let addr = self.read_rn_l(register_field)?;
        let value = self.read_abs24_b((addr - 1) & ADDRESS_MASK)?;
        self.write_rn_l(register_field, addr - 1)?;
        Ok(value)
    }

    pub(in super::super) fn write_dec_ern_w(&mut self, register_field: u8, value: u16) -> Result<()> {
        let addr = self.read_rn_l(register_field)?;
        self.write_abs24_w((addr - 2) & ADDRESS_MASK, value)?;
        self.write_rn_l(register_field, addr - 2)?;
        Ok(())
    }

    pub(in super::super) fn read_dec_ern_w(&mut self, register_field: u8) -> Result<u16> {
        let addr = self.read_rn_l(register_field)?;
        let value = self.read_abs24_w((addr - 2) & ADDRESS_MASK)?;
        self.write_rn_l(register_field, addr - 2)?;
        Ok(value)
    }

    pub(in super::super) fn write_dec_ern_l(&mut self, register_field: u8, value: u32) -> Result<()> {
        let addr = self.read_rn_l(register_field)?;
        self.write_abs24_l((addr - 4) & ADDRESS_MASK, value)?;
        self.write_rn_l(register_field, addr - 4)?;
        Ok(())
    }

    pub(in super::super) fn read_dec_ern_l(&mut self, register_field: u8) -> Result<u32> {
        let addr = self.read_rn_l(register_field)?;
        let value = self.read_abs24_l((addr - 4) & ADDRESS_MASK)?;
        self.write_rn_l(register_field, addr - 4)?;
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    #[test]
    fn test_write_dec_ern_b() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xffff11;
        cpu.write_dec_ern_b(0, 0xff).unwrap();

        assert_eq!(cpu.read_abs24_b(0xffff10).unwrap(), 0xff);
        assert_eq!(cpu.er[0], 0xffff10)
    }

    #[test]
    fn test_read_dec_ern_b() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xffff11;
        cpu.write_abs24_b(0xffff10, 0xff).unwrap();
        assert_eq!(cpu.read_dec_ern_b(0).unwrap(), 0xff);
        assert_eq!(cpu.er[0], 0xffff10)
    }

    #[test]
    fn test_write_dec_ern_w() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xffff12;
        cpu.write_dec_ern_w(0, 0x0fff).unwrap();

        assert_eq!(cpu.read_abs24_w(0xffff10).unwrap(), 0x0fff);
        assert_eq!(cpu.er[0], 0xffff10)
    }

    #[test]
    fn test_read_dec_ern_w() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xffff12;
        cpu.write_abs24_w(0xffff10, 0x0fff).unwrap();
        assert_eq!(cpu.read_dec_ern_w(0).unwrap(), 0x0fff);
        assert_eq!(cpu.er[0], 0xffff10)
    }

    #[test]
    fn test_write_dec_ern_l() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xffff14;
        cpu.write_dec_ern_l(0, 0x0f0fff0f).unwrap();

        assert_eq!(cpu.read_abs24_l(0xffff10).unwrap(), 0x0f0fff0f);
        assert_eq!(cpu.er[0], 0xffff10)
    }

    #[test]
    fn test_read_dec_ern_l() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xffff14;
        cpu.write_abs24_l(0xffff10, 0x0f0fff0f).unwrap();
        assert_eq!(cpu.read_dec_ern_l(0).unwrap(), 0x0f0fff0f);
        assert_eq!(cpu.er[0], 0xffff10)
    }
}
