use crate::cpu::Cpu;
use anyhow::{Context as _, Result};

impl Cpu {
    pub(in super::super) fn get_addr_abs8(&self, addr: u8) -> u32 {
        0xffff00 | addr as u32
    }

    pub(in super::super) fn write_abs8_b(&mut self, addr: u8, value: u8) -> Result<()> {
        let real_addr = self.get_addr_abs8(addr);
        self.bus
            .write(real_addr, value)
            .with_context(|| format!("addr [{:x}] value [{:x}]", addr, value))?;
        Ok(())
    }

    pub(in super::super) fn read_abs8_b(&self, addr: u8) -> Result<u8> {
        let real_addr = self.get_addr_abs8(addr);
        self.bus.read(real_addr).with_context(|| format!("addr [{:x}]", addr))
    }

    pub(in super::super) fn get_addr_abs16(&self, addr: u16) -> u32 {
        if addr & 0x8000 == 0x0000 {
            addr as u32
        } else {
            0xff0000 | addr as u32
        }
    }

    pub(in super::super) fn write_abs16_b(&mut self, addr: u16, value: u8) -> Result<()> {
        let real_addr = self.get_addr_abs16(addr);
        self.bus
            .write(real_addr, value)
            .with_context(|| format!("addr [{:x}] value [{:x}]", addr, value))?;
        Ok(())
    }

    pub(in super::super) fn read_abs16_b(&self, addr: u16) -> Result<u8> {
        let real_addr = self.get_addr_abs16(addr);
        return self.bus.read(real_addr).with_context(|| format!("addr [{:x}]", addr));
    }

    pub(in super::super) fn write_abs24_b(&mut self, addr: u32, value: u8) -> Result<()> {
        self.bus
            .write(addr, value)
            .with_context(|| format!("addr [{:x}] value [{:x}]", addr, value))?;
        Ok(())
    }

    pub(in super::super) fn read_abs24_b(&self, addr: u32) -> Result<u8> {
        self.bus.read(addr).with_context(|| format!("addr [{:x}]", addr))
    }

    pub(in super::super) fn write_abs8_w(&mut self, addr: u8, value: u16) -> Result<()> {
        let real_addr = self.get_addr_abs8(addr);
        self.bus
            .write(real_addr, (value >> 8) as u8)
            .with_context(|| format!("addr [{:x}] value [{:x}]", addr, value))?;
        self.bus
            .write(real_addr + 1, value as u8)
            .with_context(|| format!("addr [{:x}] value [{:x}]", addr, value))?;
        Ok(())
    }

    pub(in super::super) fn read_abs8_w(&self, addr: u8) -> Result<u16> {
        let real_addr = self.get_addr_abs8(addr);
        Ok(
            (self.bus.read(real_addr).with_context(|| format!("addr [{:x}]", addr))? as u16) << 8
                | self.bus.read(real_addr + 1).with_context(|| format!("addr [{:x}]", addr))? as u16,
        )
    }

    #[inline]
    fn _write_abs16_w(&mut self, addr: u16, value: u16) -> Result<()> {
        let real_addr = self.get_addr_abs16(addr);
        self.bus.write(real_addr, (value >> 8) as u8)?;
        self.bus.write(real_addr + 1, value as u8)?;
        Ok(())
    }

    pub(in super::super) fn write_abs16_w(&mut self, addr: u16, value: u16) -> Result<()> {
        self._write_abs16_w(addr, value)
            .with_context(|| format!("addr [{:x}] value [{:x}]", addr, value))
    }

    #[inline]
    fn _read_abs16_w(&self, addr: u16) -> Result<u16> {
        let real_addr = self.get_addr_abs16(addr);
        return Ok((self.bus.read(real_addr)? as u16) << 8 | self.bus.read(real_addr + 1)? as u16);
    }

    pub(in super::super) fn read_abs16_w(&self, addr: u16) -> Result<u16> {
        self._read_abs16_w(addr).with_context(|| format!("addr [{:x}]", addr))
    }

    pub(in super::super) fn write_abs24_w(&mut self, addr: u32, value: u16) -> Result<()> {
        self.bus
            .write(addr, (value >> 8) as u8)
            .with_context(|| format!("addr [{:x}] value [{:x}]", addr, value))?;
        self.bus
            .write(addr + 1, value as u8)
            .with_context(|| format!("addr [{:x}] value [{:x}]", addr, value))?;
        Ok(())
    }

    pub(in super::super) fn read_abs24_w(&self, addr: u32) -> Result<u16> {
        Ok((self.bus.read(addr).with_context(|| format!("addr [{:x}]", addr))? as u16) << 8
            | self.bus.read(addr + 1).with_context(|| format!("addr [{:x}]", addr))? as u16)
    }

    pub(in super::super) fn write_abs8_l(&mut self, addr: u8, value: u32) -> Result<()> {
        let real_addr = self.get_addr_abs8(addr);
        self.write_abs24_w(real_addr, (value >> 16) as u16)
            .with_context(|| format!("addr [{:x}] value [{:x}]", addr, value))?;
        self.write_abs24_w(real_addr + 2, value as u16)
            .with_context(|| format!("addr [{:x}] value [{:x}]", addr, value))?;
        Ok(())
    }

    pub(in super::super) fn read_abs8_l(&self, addr: u8) -> Result<u32> {
        let real_addr = self.get_addr_abs8(addr);
        Ok(
            (self.read_abs24_w(real_addr).with_context(|| format!("addr [{:x}]", addr))? as u32) << 16
                | self.read_abs24_w(real_addr + 2).with_context(|| format!("addr [{:x}]", addr))? as u32,
        )
    }

    #[inline]
    pub(in super::super) fn _write_abs16_l(&mut self, addr: u16, value: u32) -> Result<()> {
        let real_addr = self.get_addr_abs16(addr);
        self.write_abs24_w(real_addr, (value >> 16) as u16)?;
        self.write_abs24_w(real_addr + 2, value as u16)?;
        Ok(())
    }

    pub(in super::super) fn write_abs16_l(&mut self, addr: u16, value: u32) -> Result<()> {
        self._write_abs16_l(addr, value)
            .with_context(|| format!("addr [{:x}] value [{:x}]", addr, value))
    }

    fn _read_abs16_l(&self, addr: u16) -> Result<u32> {
        let real_addr = self.get_addr_abs16(addr);
        return Ok((self.read_abs24_w(real_addr)? as u32) << 16 | self.read_abs24_w(real_addr + 2)? as u32);
    }

    pub(in super::super) fn read_abs16_l(&self, addr: u16) -> Result<u32> {
        self._read_abs16_l(addr).with_context(|| format!("addr [{:x}]", addr))
    }

    pub(in super::super) fn write_abs24_l(&mut self, addr: u32, value: u32) -> Result<()> {
        self.write_abs24_w(addr, (value >> 16) as u16)
            .with_context(|| format!("addr [{:x}] value [{:x}]", addr, value))?;
        self.write_abs24_w(addr + 2, value as u16)
            .with_context(|| format!("addr [{:x}] value [{:x}]", addr, value))?;
        Ok(())
    }

    pub(in super::super) fn read_abs24_l(&self, addr: u32) -> Result<u32> {
        Ok(
            (self.read_abs24_w(addr).with_context(|| format!("addr [{:x}]", addr))? as u32) << 16
                | self.read_abs24_w(addr + 2).with_context(|| format!("addr [{:x}]", addr))? as u32,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    #[test]
    fn test_write_abs8_b() {
        let mut cpu = Cpu::new();
        cpu.write_abs8_b(0x10, 0xff).unwrap();
        cpu.write_abs8_b(0x1f, 0xff).unwrap();
        assert_eq!(cpu.bus.read(0xffff10).unwrap(), 0xff);
        assert_eq!(cpu.bus.read(0xffff1f).unwrap(), 0xff);
    }

    #[test]
    fn test_read_abs8_b() {
        let mut cpu = Cpu::new();
        cpu.bus.write(0xffff10, 0xff).unwrap();
        cpu.bus.write(0xffff1f, 0xff).unwrap();
        assert_eq!(cpu.read_abs8_b(0x10).unwrap(), 0xff);
        assert_eq!(cpu.read_abs8_b(0x1f).unwrap(), 0xff);
    }

    #[test]
    fn test_write_abs16_b() {
        let mut cpu = Cpu::new();
        cpu.write_abs16_b(0xff10, 0xff).unwrap();
        cpu.write_abs16_b(0xff1e, 0xff).unwrap();
        assert_eq!(cpu.bus.read(0xffff10).unwrap(), 0xff);
        assert_eq!(cpu.bus.read(0xffff1e).unwrap(), 0xff);
    }

    #[test]
    fn test_read_abs16_b() {
        let mut cpu = Cpu::new();
        cpu.bus.write(0xffff10, 0xff).unwrap();
        cpu.bus.write(0xffff1e, 0xff).unwrap();
        assert_eq!(cpu.read_abs16_b(0xff10).unwrap(), 0xff);
        assert_eq!(cpu.read_abs16_b(0xff1e).unwrap(), 0xff);
    }

    #[test]
    fn test_write_abs24_b() {
        let mut cpu = Cpu::new();
        cpu.write_abs24_b(0xffff10, 0xff).unwrap();
        cpu.write_abs24_b(0xffff1e, 0xff).unwrap();
        assert_eq!(cpu.bus.read(0xffff10).unwrap(), 0xff);
        assert_eq!(cpu.bus.read(0xffff1e).unwrap(), 0xff);
    }

    #[test]
    fn test_read_abs24_b() {
        let mut cpu = Cpu::new();
        cpu.bus.write(0xffff10, 0xff).unwrap();
        cpu.bus.write(0xffff1e, 0xff).unwrap();
        assert_eq!(cpu.read_abs24_b(0xffff10).unwrap(), 0xff);
        assert_eq!(cpu.read_abs24_b(0xffff1e).unwrap(), 0xff);
    }

    #[test]
    fn test_write_abs8_w() {
        {
            let mut cpu = Cpu::new();
            cpu.write_abs8_w(0x10, 0x0fff).unwrap();
            assert_eq!(
                (cpu.bus.read(0xffff10).unwrap() as u16) << 8 | cpu.bus.read(0xffff11).unwrap() as u16,
                0x0fff
            );
        }

        {
            let mut cpu = Cpu::new();
            cpu.write_abs8_w(0x01, 0x0fff).unwrap();
            assert_eq!(
                (cpu.bus.read(0xffff01).unwrap() as u16) << 8 | cpu.bus.read(0xffff02).unwrap() as u16,
                0x0fff
            );
        }
    }

    #[test]
    fn test_read_abs8_w() {
        let mut cpu = Cpu::new();
        cpu.bus.write(0xffff10, 0x0f).unwrap();
        cpu.bus.write(0xffff11, 0xff).unwrap();
        assert_eq!(cpu.read_abs8_w(0x10).unwrap(), 0x0fff);
    }

    // アドレスの最上位ビットが1のときのみ
    #[test]
    fn test_write_abs16_w() {
        let mut cpu = Cpu::new();
        cpu.write_abs16_w(0xff10, 0x0fff).unwrap();
        assert_eq!(
            (cpu.bus.read(0xffff10).unwrap() as u16) << 8 | cpu.bus.read(0xffff11).unwrap() as u16,
            0x0fff
        );
    }

    // アドレスの最上位ビットが1のときのみ
    #[test]
    fn test_read_abs16_w() {
        let mut cpu = Cpu::new();
        cpu.bus.write(0xffff00, 0x0f).unwrap();
        cpu.bus.write(0xffff01, 0x0f).unwrap();
        assert_eq!(cpu.read_abs16_w(0xff00).unwrap(), 0x0f0f);

        cpu.bus.write(0xffff10, 0x0f).unwrap();
        cpu.bus.write(0xffff11, 0xff).unwrap();
        assert_eq!(cpu.read_abs16_w(0xff10).unwrap(), 0x0fff);
    }

    #[test]
    fn test_write_abs24_w() {
        let mut cpu = Cpu::new();
        cpu.write_abs24_w(0xffff10, 0x0fff).unwrap();
        assert_eq!(
            (cpu.bus.read(0xffff10).unwrap() as u16) << 8 | cpu.bus.read(0xffff11).unwrap() as u16,
            0x0fff
        );
    }

    #[test]
    fn test_read_abs24_w() {
        let mut cpu = Cpu::new();
        cpu.bus.write(0xffff00, 0x0f).unwrap();
        cpu.bus.write(0xffff01, 0x0f).unwrap();
        assert_eq!(cpu.read_abs24_w(0xffff00).unwrap(), 0x0f0f);

        cpu.bus.write(0xffff10, 0x0f).unwrap();
        cpu.bus.write(0xffff11, 0xff).unwrap();
        assert_eq!(cpu.read_abs24_w(0xffff10).unwrap(), 0x0fff);
    }

    #[test]
    fn test_write_abs8_l() {
        let mut cpu = Cpu::new();
        cpu.write_abs8_l(0x10, 0x0f0fff0f).unwrap();
        assert_eq!(
            (cpu.read_abs24_w(0xffff10).unwrap() as u32) << 16 | cpu.read_abs24_w(0xffff12).unwrap() as u32,
            0x0f0fff0f
        );
    }

    #[test]
    fn test_read_abs8_l() {
        let mut cpu = Cpu::new();
        cpu.write_abs24_w(0xffff10, 0x0f0f).unwrap();
        cpu.write_abs24_w(0xffff12, 0xff0f).unwrap();
        assert_eq!(cpu.read_abs8_l(0x10).unwrap(), 0x0f0fff0f);
    }

    // アドレスの最上位ビットが1のときのみ
    #[test]
    fn test_write_abs16_l() {
        let mut cpu = Cpu::new();
        cpu.write_abs16_l(0xff10, 0x0f0fff0f).unwrap();
        assert_eq!(
            (cpu.read_abs24_w(0xffff10).unwrap() as u32) << 16 | cpu.read_abs24_w(0xffff12).unwrap() as u32,
            0x0f0fff0f
        );
    }

    // アドレスの最上位ビットが1のときのみ
    #[test]
    fn test_read_abs16_l() {
        let mut cpu = Cpu::new();
        cpu.write_abs24_w(0xffff00, 0x0f0f).unwrap();
        cpu.write_abs24_w(0xffff02, 0xff0f).unwrap();
        assert_eq!(cpu.read_abs16_l(0xff00).unwrap(), 0x0f0fff0f);
    }
    #[test]
    fn test_write_abs24_l() {
        let mut cpu = Cpu::new();
        cpu.write_abs24_l(0xffff10, 0x0f0fff0f).unwrap();
        assert_eq!(
            (cpu.read_abs24_w(0xffff10).unwrap() as u32) << 16 | cpu.read_abs24_w(0xffff12).unwrap() as u32,
            0x0f0fff0f
        );
    }

    #[test]
    fn test_read_abs24_l() {
        let mut cpu = Cpu::new();
        cpu.write_abs24_w(0xffff00, 0x0f0f).unwrap();
        cpu.write_abs24_w(0xffff02, 0xff0f).unwrap();
        assert_eq!(cpu.read_abs24_l(0xffff00).unwrap(), 0x0f0fff0f);
    }
}
