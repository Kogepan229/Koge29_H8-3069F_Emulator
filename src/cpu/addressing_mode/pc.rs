use super::*;
use anyhow::{anyhow, Context as _, Result};

impl<'a> Cpu<'a> {
    pub(in super::super) fn pc_disp8(&mut self, disp: u8) -> Result<()> {
        let mut _disp = disp as u32;
        if (disp & 0x80) == 0x80 {
            _disp |= 0xffffff00
        }
        self.pc = self.pc.checked_add_signed(_disp as i32).ok_or_else(|| {
            anyhow!(
                "attempt to add with overflow [{:x} + {:x}]",
                self.pc,
                _disp as i32
            )
        })?;
        if self.pc % 2 != 0 {
            bail!("pc is not odd. [{:x}]]", self.pc);
        }
        Ok(())
    }

    pub(in super::super) fn pc_disp16(&mut self, disp: u16) -> Result<()> {
        let mut _disp = disp as u32;
        if (disp & 0x8000) == 0x8000 {
            _disp |= 0xffff0000
        }
        self.pc = self.pc.checked_add_signed(_disp as i32).ok_or_else(|| {
            anyhow!(
                "attempt to add with overflow [{:x} + {:x}]",
                self.pc,
                _disp as i32
            )
        })?;
        if self.pc % 2 != 0 {
            bail!("pc is not odd. [{:x}]]", self.pc);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{cpu::Cpu, mcu::Mcu};

    #[test]
    fn pc_disp8() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.pc = 0xffff02;
        cpu.pc_disp8(0xfe).unwrap();
        assert_eq!(cpu.pc, 0xffff00);

        cpu.pc = 0xffff02;
        cpu.pc_disp8(0x8).unwrap();
        assert_eq!(cpu.pc, 0xffff0a);
    }

    #[test]
    fn pc_disp16() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.pc = 0xffff02;
        cpu.pc_disp16(0xfffe).unwrap();
        assert_eq!(cpu.pc, 0xffff00);

        cpu.pc = 0xffff02;
        cpu.pc_disp16(0x8).unwrap();
        assert_eq!(cpu.pc, 0xffff0a);
    }
}
