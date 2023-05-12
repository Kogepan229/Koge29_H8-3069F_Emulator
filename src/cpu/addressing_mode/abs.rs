use super::*;

impl<'a> Cpu<'a> {
    pub(in super::super) fn write_abs8_b(&mut self, addr: u8, value: u8) {
        self.mcu.write(0xffff00 | addr as u32, value).unwrap();
    }

    pub(in super::super) fn read_abs8_b(&self, addr: u8) -> u8 {
        self.mcu.read(0xffff00 | addr as u32).unwrap()
    }

    pub(in super::super) fn write_abs16_b(&mut self, addr: u16, value: u8) {
        if addr & 0x8000 == 0x0000 {
            self.mcu.write(addr as u32, value).unwrap();
        } else {
            self.mcu.write(0xff0000 | addr as u32, value).unwrap();
        }
    }

    pub(in super::super) fn read_abs16_b(&self, addr: u16) -> u8 {
        if addr & 0x8000 == 0x0000 {
            return self.mcu.read(addr as u32).unwrap();
        } else {
            return self.mcu.read(0xff0000 | addr as u32).unwrap();
        }
    }

    pub(in super::super) fn write_abs24_b(&mut self, addr: u32, value: u8) {
        self.mcu.write(addr, value).unwrap();
    }

    pub(in super::super) fn read_abs24_b(&self, addr: u32) -> u8 {
        self.mcu.read(addr).unwrap()
    }

    pub(in super::super) fn write_abs8_w(
        &mut self,
        mut addr: u8,
        value: u16,
    ) -> Result<(), AddressingError> {
        if addr % 2 != 0 {
            addr &= !1;
        }
        self.mcu
            .write(0xffff00 | addr as u32, (value >> 8) as u8)
            .unwrap();
        self.mcu
            .write((0xffff00 | addr as u32) + 1, value as u8)
            .unwrap();
        Ok(())
    }

    pub(in super::super) fn read_abs8_w(&self, mut addr: u8) -> Result<u16, AddressingError> {
        if addr % 2 != 0 {
            addr &= !1;
        }
        Ok((self.mcu.read(0xffff00 | addr as u32).unwrap() as u16) << 8
            | self.mcu.read((0xffff00 | addr as u32) + 1).unwrap() as u16)
    }

    pub(in super::super) fn write_abs16_w(
        &mut self,
        mut addr: u16,
        value: u16,
    ) -> Result<(), AddressingError> {
        if addr % 2 != 0 {
            addr &= !1;
        }
        if addr & 0x8000 == 0x0000 {
            self.mcu.write(addr as u32, (value >> 8) as u8).unwrap();
            self.mcu.write((addr + 1) as u32, value as u8).unwrap();
        } else {
            self.mcu
                .write(0xff0000 | addr as u32, (value >> 8) as u8)
                .unwrap();
            self.mcu
                .write((0xff0000 | addr as u32) + 1, value as u8)
                .unwrap();
        }
        Ok(())
    }

    pub(in super::super) fn read_abs16_w(&self, mut addr: u16) -> Result<u16, AddressingError> {
        if addr % 2 != 0 {
            addr &= !1;
        }
        if addr & 0x8000 == 0x0000 {
            return Ok((self.mcu.read(addr as u32).unwrap() as u16) << 8
                | self.mcu.read((addr + 1) as u32).unwrap() as u16);
        } else {
            return Ok((self.mcu.read(0xff0000 | addr as u32).unwrap() as u16) << 8
                | self.mcu.read((0xff0000 | addr as u32) + 1).unwrap() as u16);
        }
    }

    pub(in super::super) fn write_abs24_w(
        &mut self,
        mut addr: u32,
        value: u16,
    ) -> Result<(), AddressingError> {
        if addr % 2 != 0 {
            addr &= !1;
        }
        self.mcu.write(addr, (value >> 8) as u8).unwrap();
        self.mcu.write(addr + 1, value as u8).unwrap();
        Ok(())
    }

    pub(in super::super) fn read_abs24_w(&self, mut addr: u32) -> Result<u16, AddressingError> {
        if addr % 2 != 0 {
            addr &= !1;
        }
        Ok((self.mcu.read(addr).unwrap() as u16) << 8 | self.mcu.read(addr + 1).unwrap() as u16)
    }

    pub(in super::super) fn write_abs8_l(
        &mut self,
        mut addr: u8,
        value: u32,
    ) -> Result<(), AddressingError> {
        if addr % 2 != 0 {
            addr &= !1;
        }
        self.write_abs24_w(0xffff00 | addr as u32, (value >> 16) as u16)
            .unwrap();
        self.write_abs24_w((0xffff00 | addr as u32) + 2, value as u16)
            .unwrap();
        Ok(())
    }

    pub(in super::super) fn read_abs8_l(&self, mut addr: u8) -> Result<u32, AddressingError> {
        if addr % 2 != 0 {
            addr &= !1;
        }
        Ok(
            (self.read_abs24_w(0xffff00 | addr as u32).unwrap() as u32) << 16
                | self.read_abs24_w((0xffff00 | addr as u32) + 2).unwrap() as u32,
        )
    }

    pub(in super::super) fn write_abs16_l(
        &mut self,
        mut addr: u16,
        value: u32,
    ) -> Result<(), AddressingError> {
        if addr % 2 != 0 {
            addr &= !1;
        }
        if addr & 0x8000 == 0x0000 {
            self.write_abs24_w(addr as u32, (value >> 16) as u16)
                .unwrap();
            self.write_abs24_w((addr + 2) as u32, value as u16).unwrap();
        } else {
            self.write_abs24_w(0xff0000 | addr as u32, (value >> 16) as u16)
                .unwrap();
            self.write_abs24_w((0xff0000 | addr as u32) + 2, value as u16)
                .unwrap();
        }
        Ok(())
    }

    pub(in super::super) fn read_abs16_l(&self, mut addr: u16) -> Result<u32, AddressingError> {
        if addr % 2 != 0 {
            addr &= !1;
        }
        if addr & 0x8000 == 0x0000 {
            return Ok((self.read_abs24_w(addr as u32).unwrap() as u32) << 16
                | self.read_abs24_w((addr + 2) as u32).unwrap() as u32);
        } else {
            return Ok(
                (self.read_abs24_w(0xff0000 | addr as u32).unwrap() as u32) << 16
                    | self.read_abs24_w((0xff0000 | addr as u32) + 2).unwrap() as u32,
            );
        }
    }

    pub(in super::super) fn write_abs24_l(
        &mut self,
        mut addr: u32,
        value: u32,
    ) -> Result<(), AddressingError> {
        if addr % 2 != 0 {
            addr &= !1;
        }
        self.write_abs24_w(addr, (value >> 16) as u16).unwrap();
        self.write_abs24_w(addr + 2, value as u16).unwrap();
        Ok(())
    }

    pub(in super::super) fn read_abs24_l(&self, mut addr: u32) -> Result<u32, AddressingError> {
        if addr % 2 != 0 {
            addr &= !1;
        }
        Ok((self.read_abs24_w(addr).unwrap() as u32) << 16
            | self.read_abs24_w(addr + 2).unwrap() as u32)
    }
}

#[cfg(test)]
mod tests {
    use crate::{cpu::Cpu, mcu::Mcu};

    #[test]
    fn test_write_abs8_b() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.write_abs8_b(0x10, 0xff);
        cpu.write_abs8_b(0x1f, 0xff);
        assert_eq!(cpu.mcu.read(0xffff10).unwrap(), 0xff);
        assert_eq!(cpu.mcu.read(0xffff1f).unwrap(), 0xff);
    }

    #[test]
    fn test_read_abs8_b() {
        let mut mcu = Mcu::new();
        let cpu = Cpu::new(&mut mcu);
        cpu.mcu.write(0xffff10, 0xff).unwrap();
        cpu.mcu.write(0xffff1f, 0xff).unwrap();
        assert_eq!(cpu.read_abs8_b(0x10), 0xff);
        assert_eq!(cpu.read_abs8_b(0x1f), 0xff);
    }

    #[test]
    fn test_write_abs16_b() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.write_abs16_b(0xff10, 0xff);
        cpu.write_abs16_b(0xff1f, 0xff);
        assert_eq!(cpu.mcu.read(0xffff10).unwrap(), 0xff);
        assert_eq!(cpu.mcu.read(0xffff1f).unwrap(), 0xff);
    }

    #[test]
    fn test_read_abs16_b() {
        let mut mcu = Mcu::new();
        let cpu = Cpu::new(&mut mcu);
        cpu.mcu.write(0xffff10, 0xff).unwrap();
        cpu.mcu.write(0xffff1f, 0xff).unwrap();
        assert_eq!(cpu.read_abs16_b(0xff10), 0xff);
        assert_eq!(cpu.read_abs16_b(0xff1f), 0xff);
    }

    #[test]
    fn test_write_abs24_b() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.write_abs24_b(0xffff10, 0xff);
        cpu.write_abs24_b(0xffff1f, 0xff);
        assert_eq!(cpu.mcu.read(0xffff10).unwrap(), 0xff);
        assert_eq!(cpu.mcu.read(0xffff1f).unwrap(), 0xff);
    }

    #[test]
    fn test_read_abs24_b() {
        let mut mcu = Mcu::new();
        let cpu = Cpu::new(&mut mcu);
        cpu.mcu.write(0xffff10, 0xff).unwrap();
        cpu.mcu.write(0xffff1f, 0xff).unwrap();
        assert_eq!(cpu.read_abs24_b(0xffff10), 0xff);
        assert_eq!(cpu.read_abs24_b(0xffff1f), 0xff);
    }

    #[test]
    fn test_write_abs8_w() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.write_abs8_w(0x10, 0x0fff).unwrap();
        assert_eq!(
            (cpu.mcu.read(0xffff10).unwrap() as u16) << 8 | cpu.mcu.read(0xffff11).unwrap() as u16,
            0x0fff
        );

        cpu.write_abs8_w(0x01, 0x0fff).unwrap();
        assert_eq!(
            (cpu.mcu.read(0xffff00).unwrap() as u16) << 8 | cpu.mcu.read(0xffff01).unwrap() as u16,
            0x0fff
        );
    }

    #[test]
    fn test_read_abs8_w() {
        let mut mcu = Mcu::new();
        let cpu = Cpu::new(&mut mcu);
        cpu.mcu.write(0xffff10, 0x0f).unwrap();
        cpu.mcu.write(0xffff11, 0xff).unwrap();
        assert_eq!(cpu.read_abs8_w(0x10).unwrap(), 0x0fff);
    }

    // アドレスの最上位ビットが1のときのみ
    #[test]
    fn test_write_abs16_w() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.write_abs16_w(0xff10, 0x0fff).unwrap();
        assert_eq!(
            (cpu.mcu.read(0xffff10).unwrap() as u16) << 8 | cpu.mcu.read(0xffff11).unwrap() as u16,
            0x0fff
        );
    }

    // アドレスの最上位ビットが1のときのみ
    #[test]
    fn test_read_abs16_w() {
        let mut mcu = Mcu::new();
        let cpu = Cpu::new(&mut mcu);
        cpu.mcu.write(0xffff00, 0x0f).unwrap();
        cpu.mcu.write(0xffff01, 0x0f).unwrap();
        assert_eq!(cpu.read_abs16_w(0xff00).unwrap(), 0x0f0f);

        cpu.mcu.write(0xffff10, 0x0f).unwrap();
        cpu.mcu.write(0xffff11, 0xff).unwrap();
        assert_eq!(cpu.read_abs16_w(0xff10).unwrap(), 0x0fff);
    }

    #[test]
    fn test_write_abs24_w() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.write_abs24_w(0xffff10, 0x0fff).unwrap();
        assert_eq!(
            (cpu.mcu.read(0xffff10).unwrap() as u16) << 8 | cpu.mcu.read(0xffff11).unwrap() as u16,
            0x0fff
        );
    }

    #[test]
    fn test_read_abs24_w() {
        let mut mcu = Mcu::new();
        let cpu = Cpu::new(&mut mcu);
        cpu.mcu.write(0xffff00, 0x0f).unwrap();
        cpu.mcu.write(0xffff01, 0x0f).unwrap();
        assert_eq!(cpu.read_abs24_w(0xffff00).unwrap(), 0x0f0f);

        cpu.mcu.write(0xffff10, 0x0f).unwrap();
        cpu.mcu.write(0xffff11, 0xff).unwrap();
        assert_eq!(cpu.read_abs24_w(0xffff10).unwrap(), 0x0fff);
    }

    #[test]
    fn test_write_abs8_l() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.write_abs8_l(0x10, 0x0f0fff0f).unwrap();
        assert_eq!(
            (cpu.read_abs24_w(0xffff10).unwrap() as u32) << 16
                | cpu.read_abs24_w(0xffff12).unwrap() as u32,
            0x0f0fff0f
        );
    }

    #[test]
    fn test_read_abs8_l() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.write_abs24_w(0xffff10, 0x0f0f).unwrap();
        cpu.write_abs24_w(0xffff12, 0xff0f).unwrap();
        assert_eq!(cpu.read_abs8_l(0x10).unwrap(), 0x0f0fff0f);
    }

    // アドレスの最上位ビットが1のときのみ
    #[test]
    fn test_write_abs16_l() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.write_abs16_l(0xff10, 0x0f0fff0f).unwrap();
        assert_eq!(
            (cpu.read_abs24_w(0xffff10).unwrap() as u32) << 16
                | cpu.read_abs24_w(0xffff12).unwrap() as u32,
            0x0f0fff0f
        );
    }

    // アドレスの最上位ビットが1のときのみ
    #[test]
    fn test_read_abs16_l() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.write_abs24_w(0xffff00, 0x0f0f).unwrap();
        cpu.write_abs24_w(0xffff02, 0xff0f).unwrap();
        assert_eq!(cpu.read_abs16_l(0xff00).unwrap(), 0x0f0fff0f);
    }

    #[test]
    fn test_write_abs24_l() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.write_abs24_l(0xffff10, 0x0f0fff0f).unwrap();
        assert_eq!(
            (cpu.read_abs24_w(0xffff10).unwrap() as u32) << 16
                | cpu.read_abs24_w(0xffff12).unwrap() as u32,
            0x0f0fff0f
        );
    }

    #[test]
    fn test_read_abs24_l() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.write_abs24_w(0xffff00, 0x0f0f).unwrap();
        cpu.write_abs24_w(0xffff02, 0xff0f).unwrap();
        assert_eq!(cpu.read_abs24_l(0xffff00).unwrap(), 0x0f0fff0f);
    }
}
