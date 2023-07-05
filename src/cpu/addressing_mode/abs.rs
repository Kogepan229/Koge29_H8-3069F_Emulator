use crate::cpu::Cpu;
use anyhow::{Context as _, Result};

impl Cpu {
    pub(in super::super) async fn write_abs8_b(&mut self, addr: u8, value: u8) -> Result<()> {
        self.bus
            .lock()
            .await
            .write(0xffff00 | addr as u32, value)
            .with_context(|| format!("addr [{:x}] value [{:x}]", addr, value))?;
        Ok(())
    }

    pub(in super::super) async fn read_abs8_b(&self, addr: u8) -> Result<u8> {
        self.bus
            .lock()
            .await
            .read(0xffff00 | addr as u32)
            .with_context(|| format!("addr [{:x}]", addr))
    }

    pub(in super::super) async fn write_abs16_b(&mut self, addr: u16, value: u8) -> Result<()> {
        if addr & 0x8000 == 0x0000 {
            self.bus
                .lock()
                .await
                .write(addr as u32, value)
                .with_context(|| format!("addr [{:x}] value [{:x}]", addr, value))?;
        } else {
            self.bus
                .lock()
                .await
                .write(0xff0000 | addr as u32, value)
                .with_context(|| format!("addr [{:x}] value [{:x}]", addr, value))?;
        }
        Ok(())
    }

    pub(in super::super) async fn read_abs16_b(&self, addr: u16) -> Result<u8> {
        if addr & 0x8000 == 0x0000 {
            return self
                .bus
                .lock()
                .await
                .read(addr as u32)
                .with_context(|| format!("addr [{:x}]", addr));
        } else {
            return self
                .bus
                .lock()
                .await
                .read(0xff0000 | addr as u32)
                .with_context(|| format!("addr [{:x}]", addr));
        }
    }

    pub(in super::super) async fn write_abs24_b(&mut self, addr: u32, value: u8) -> Result<()> {
        self.bus
            .lock()
            .await
            .write(addr, value)
            .with_context(|| format!("addr [{:x}] value [{:x}]", addr, value))?;
        Ok(())
    }

    pub(in super::super) async fn read_abs24_b(&self, addr: u32) -> Result<u8> {
        self.bus
            .lock()
            .await
            .read(addr)
            .with_context(|| format!("addr [{:x}]", addr))
    }

    pub(in super::super) async fn write_abs8_w(&mut self, mut addr: u8, value: u16) -> Result<()> {
        if addr % 2 != 0 {
            addr &= !1;
        }
        let mut bus_lock = self.bus.lock().await;
        bus_lock
            .write(0xffff00 | addr as u32, (value >> 8) as u8)
            .with_context(|| format!("addr [{:x}] value [{:x}]", addr, value))?;
        bus_lock
            .write((0xffff00 | addr as u32) + 1, value as u8)
            .with_context(|| format!("addr [{:x}] value [{:x}]", addr, value))?;
        Ok(())
    }

    pub(in super::super) async fn read_abs8_w(&self, mut addr: u8) -> Result<u16> {
        if addr % 2 != 0 {
            addr &= !1;
        }
        let bus_lock = self.bus.lock().await;
        Ok((bus_lock
            .read(0xffff00 | addr as u32)
            .with_context(|| format!("addr [{:x}]", addr))? as u16)
            << 8
            | bus_lock
                .read((0xffff00 | addr as u32) + 1)
                .with_context(|| format!("addr [{:x}]", addr))? as u16)
    }

    #[inline]
    async fn _write_abs16_w(&mut self, mut addr: u16, value: u16) -> Result<()> {
        if addr % 2 != 0 {
            addr &= !1;
        }
        let mut bus_lock = self.bus.lock().await;
        if addr & 0x8000 == 0x0000 {
            bus_lock.write(addr as u32, (value >> 8) as u8)?;
            bus_lock.write((addr + 1) as u32, value as u8)?;
        } else {
            bus_lock.write(0xff0000 | addr as u32, (value >> 8) as u8)?;
            bus_lock.write((0xff0000 | addr as u32) + 1, value as u8)?;
        }
        Ok(())
    }

    pub(in super::super) async fn write_abs16_w(&mut self, addr: u16, value: u16) -> Result<()> {
        self._write_abs16_w(addr, value)
            .await
            .with_context(|| format!("addr [{:x}] value [{:x}]", addr, value))
    }

    #[inline]
    async fn _read_abs16_w(&self, mut addr: u16) -> Result<u16> {
        if addr % 2 != 0 {
            addr &= !1;
        }
        let bus_lock = self.bus.lock().await;
        if addr & 0x8000 == 0x0000 {
            return Ok((bus_lock.read(addr as u32)? as u16) << 8
                | bus_lock.read((addr + 1) as u32)? as u16);
        } else {
            return Ok((bus_lock.read(0xff0000 | addr as u32)? as u16) << 8
                | bus_lock.read((0xff0000 | addr as u32) + 1)? as u16);
        }
    }

    pub(in super::super) async fn read_abs16_w(&self, addr: u16) -> Result<u16> {
        self._read_abs16_w(addr)
            .await
            .with_context(|| format!("addr [{:x}]", addr))
    }

    pub(in super::super) async fn write_abs24_w(
        &mut self,
        mut addr: u32,
        value: u16,
    ) -> Result<()> {
        if addr % 2 != 0 {
            addr &= !1;
        }
        self.bus
            .lock()
            .await
            .write(addr, (value >> 8) as u8)
            .with_context(|| format!("addr [{:x}] value [{:x}]", addr, value))?;
        self.bus
            .lock()
            .await
            .write(addr + 1, value as u8)
            .with_context(|| format!("addr [{:x}] value [{:x}]", addr, value))?;
        Ok(())
    }

    pub(in super::super) async fn read_abs24_w(&self, mut addr: u32) -> Result<u16> {
        if addr % 2 != 0 {
            addr &= !1;
        }
        let bus_lock = self.bus.lock().await;
        Ok((bus_lock
            .read(addr)
            .with_context(|| format!("addr [{:x}]", addr))? as u16)
            << 8
            | bus_lock
                .read(addr + 1)
                .with_context(|| format!("addr [{:x}]", addr))? as u16)
    }

    pub(in super::super) async fn write_abs8_l(&mut self, mut addr: u8, value: u32) -> Result<()> {
        if addr % 2 != 0 {
            addr &= !1;
        }
        self.write_abs24_w(0xffff00 | addr as u32, (value >> 16) as u16)
            .await
            .with_context(|| format!("addr [{:x}] value [{:x}]", addr, value))?;
        self.write_abs24_w((0xffff00 | addr as u32) + 2, value as u16)
            .await
            .with_context(|| format!("addr [{:x}] value [{:x}]", addr, value))?;
        Ok(())
    }

    pub(in super::super) async fn read_abs8_l(&self, mut addr: u8) -> Result<u32> {
        if addr % 2 != 0 {
            addr &= !1;
        }
        Ok((self
            .read_abs24_w(0xffff00 | addr as u32)
            .await
            .with_context(|| format!("addr [{:x}]", addr))? as u32)
            << 16
            | self
                .read_abs24_w((0xffff00 | addr as u32) + 2)
                .await
                .with_context(|| format!("addr [{:x}]", addr))? as u32)
    }

    #[inline]
    pub(in super::super) async fn _write_abs16_l(
        &mut self,
        mut addr: u16,
        value: u32,
    ) -> Result<()> {
        if addr % 2 != 0 {
            addr &= !1;
        }
        if addr & 0x8000 == 0x0000 {
            self.write_abs24_w(addr as u32, (value >> 16) as u16)
                .await?;
            self.write_abs24_w((addr + 2) as u32, value as u16).await?;
        } else {
            self.write_abs24_w(0xff0000 | addr as u32, (value >> 16) as u16)
                .await?;
            self.write_abs24_w((0xff0000 | addr as u32) + 2, value as u16)
                .await?;
        }
        Ok(())
    }

    pub(in super::super) async fn write_abs16_l(&mut self, addr: u16, value: u32) -> Result<()> {
        self._write_abs16_l(addr, value)
            .await
            .with_context(|| format!("addr [{:x}] value [{:x}]", addr, value))
    }

    async fn _read_abs16_l(&self, mut addr: u16) -> Result<u32> {
        if addr % 2 != 0 {
            addr &= !1;
        }
        if addr & 0x8000 == 0x0000 {
            return Ok((self.read_abs24_w(addr as u32).await? as u32) << 16
                | self.read_abs24_w((addr + 2) as u32).await? as u32);
        } else {
            return Ok(
                (self.read_abs24_w(0xff0000 | addr as u32).await? as u32) << 16
                    | self.read_abs24_w((0xff0000 | addr as u32) + 2).await? as u32,
            );
        }
    }

    pub(in super::super) async fn read_abs16_l(&self, addr: u16) -> Result<u32> {
        self._read_abs16_l(addr)
            .await
            .with_context(|| format!("addr [{:x}]", addr))
    }

    pub(in super::super) async fn write_abs24_l(
        &mut self,
        mut addr: u32,
        value: u32,
    ) -> Result<()> {
        if addr % 2 != 0 {
            addr &= !1;
        }
        self.write_abs24_w(addr, (value >> 16) as u16)
            .await
            .with_context(|| format!("addr [{:x}] value [{:x}]", addr, value))?;
        self.write_abs24_w(addr + 2, value as u16)
            .await
            .with_context(|| format!("addr [{:x}] value [{:x}]", addr, value))?;
        Ok(())
    }

    pub(in super::super) async fn read_abs24_l(&self, mut addr: u32) -> Result<u32> {
        if addr % 2 != 0 {
            addr &= !1;
        }
        Ok((self
            .read_abs24_w(addr)
            .await
            .with_context(|| format!("addr [{:x}]", addr))? as u32)
            << 16
            | self
                .read_abs24_w(addr + 2)
                .await
                .with_context(|| format!("addr [{:x}]", addr))? as u32)
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    #[tokio::test]
    async fn test_write_abs8_b() {
        let mut cpu = Cpu::new();
        cpu.write_abs8_b(0x10, 0xff).await.unwrap();
        cpu.write_abs8_b(0x1f, 0xff).await.unwrap();
        assert_eq!(cpu.bus.lock().await.read(0xffff10).unwrap(), 0xff);
        assert_eq!(cpu.bus.lock().await.read(0xffff1f).unwrap(), 0xff);
    }

    #[tokio::test]
    async fn test_read_abs8_b() {
        let mut cpu = Cpu::new();
        cpu.bus.lock().await.write(0xffff10, 0xff).unwrap();
        cpu.bus.lock().await.write(0xffff1f, 0xff).unwrap();
        assert_eq!(cpu.read_abs8_b(0x10).await.unwrap(), 0xff);
        assert_eq!(cpu.read_abs8_b(0x1f).await.unwrap(), 0xff);
    }

    #[tokio::test]
    async fn test_write_abs16_b() {
        let mut cpu = Cpu::new();
        cpu.write_abs16_b(0xff10, 0xff).await.unwrap();
        cpu.write_abs16_b(0xff1f, 0xff).await.unwrap();
        assert_eq!(cpu.bus.lock().await.read(0xffff10).unwrap(), 0xff);
        assert_eq!(cpu.bus.lock().await.read(0xffff1f).unwrap(), 0xff);
    }

    #[tokio::test]
    async fn test_read_abs16_b() {
        let cpu = Cpu::new();
        cpu.bus.lock().await.write(0xffff10, 0xff).unwrap();
        cpu.bus.lock().await.write(0xffff1f, 0xff).unwrap();
        assert_eq!(cpu.read_abs16_b(0xff10).await.unwrap(), 0xff);
        assert_eq!(cpu.read_abs16_b(0xff1f).await.unwrap(), 0xff);
    }

    #[tokio::test]
    async fn test_write_abs24_b() {
        let mut cpu = Cpu::new();
        cpu.write_abs24_b(0xffff10, 0xff).await.unwrap();
        cpu.write_abs24_b(0xffff1f, 0xff).await.unwrap();
        assert_eq!(cpu.bus.lock().await.read(0xffff10).unwrap(), 0xff);
        assert_eq!(cpu.bus.lock().await.read(0xffff1f).unwrap(), 0xff);
    }

    #[tokio::test]
    async fn test_read_abs24_b() {
        let cpu = Cpu::new();
        cpu.bus.lock().await.write(0xffff10, 0xff).unwrap();
        cpu.bus.lock().await.write(0xffff1f, 0xff).unwrap();
        assert_eq!(cpu.read_abs24_b(0xffff10).await.unwrap(), 0xff);
        assert_eq!(cpu.read_abs24_b(0xffff1f).await.unwrap(), 0xff);
    }

    #[tokio::test]
    async fn test_write_abs8_w() {
        let mut cpu = Cpu::new();
        cpu.write_abs8_w(0x10, 0x0fff).await.unwrap();
        {
            let bus_lock = cpu.bus.lock().await;
            assert_eq!(
                (bus_lock.read(0xffff10).unwrap() as u16) << 8
                    | bus_lock.read(0xffff11).unwrap() as u16,
                0x0fff
            );
        }

        cpu.write_abs8_w(0x01, 0x0fff).await.unwrap();
        {
            let bus_lock = cpu.bus.lock().await;
            assert_eq!(
                (bus_lock.read(0xffff00).unwrap() as u16) << 8
                    | bus_lock.read(0xffff01).unwrap() as u16,
                0x0fff
            );
        }
    }

    #[tokio::test]
    async fn test_read_abs8_w() {
        let cpu = Cpu::new();
        cpu.bus.lock().await.write(0xffff10, 0x0f).unwrap();
        cpu.bus.lock().await.write(0xffff11, 0xff).unwrap();
        assert_eq!(cpu.read_abs8_w(0x10).await.unwrap(), 0x0fff);
    }

    // アドレスの最上位ビットが1のときのみ
    #[tokio::test]
    async fn test_write_abs16_w() {
        let mut cpu = Cpu::new();
        cpu.write_abs16_w(0xff10, 0x0fff).await.unwrap();
        let bus_lock = cpu.bus.lock().await;
        assert_eq!(
            (bus_lock.read(0xffff10).unwrap() as u16) << 8
                | bus_lock.read(0xffff11).unwrap() as u16,
            0x0fff
        );
    }

    // アドレスの最上位ビットが1のときのみ
    #[tokio::test]
    async fn test_read_abs16_w() {
        let cpu = Cpu::new();
        cpu.bus.lock().await.write(0xffff00, 0x0f).unwrap();
        cpu.bus.lock().await.write(0xffff01, 0x0f).unwrap();
        assert_eq!(cpu.read_abs16_w(0xff00).await.unwrap(), 0x0f0f);

        cpu.bus.lock().await.write(0xffff10, 0x0f).unwrap();
        cpu.bus.lock().await.write(0xffff11, 0xff).unwrap();
        assert_eq!(cpu.read_abs16_w(0xff10).await.unwrap(), 0x0fff);
    }

    #[tokio::test]
    async fn test_write_abs24_w() {
        let mut cpu = Cpu::new();
        cpu.write_abs24_w(0xffff10, 0x0fff).await.unwrap();
        let bus_lock = cpu.bus.lock().await;
        assert_eq!(
            (bus_lock.read(0xffff10).unwrap() as u16) << 8
                | bus_lock.read(0xffff11).unwrap() as u16,
            0x0fff
        );
    }

    #[tokio::test]
    async fn test_read_abs24_w() {
        let cpu = Cpu::new();
        cpu.bus.lock().await.write(0xffff00, 0x0f).unwrap();
        cpu.bus.lock().await.write(0xffff01, 0x0f).unwrap();
        assert_eq!(cpu.read_abs24_w(0xffff00).await.unwrap(), 0x0f0f);

        cpu.bus.lock().await.write(0xffff10, 0x0f).unwrap();
        cpu.bus.lock().await.write(0xffff11, 0xff).unwrap();
        assert_eq!(cpu.read_abs24_w(0xffff10).await.unwrap(), 0x0fff);
    }

    #[tokio::test]
    async fn test_write_abs8_l() {
        let mut cpu = Cpu::new();
        cpu.write_abs8_l(0x10, 0x0f0fff0f).await.unwrap();
        assert_eq!(
            (cpu.read_abs24_w(0xffff10).await.unwrap() as u32) << 16
                | cpu.read_abs24_w(0xffff12).await.unwrap() as u32,
            0x0f0fff0f
        );
    }

    #[tokio::test]
    async fn test_read_abs8_l() {
        let mut cpu = Cpu::new();
        cpu.write_abs24_w(0xffff10, 0x0f0f).await.unwrap();
        cpu.write_abs24_w(0xffff12, 0xff0f).await.unwrap();
        assert_eq!(cpu.read_abs8_l(0x10).await.unwrap(), 0x0f0fff0f);
    }

    // アドレスの最上位ビットが1のときのみ
    #[tokio::test]
    async fn test_write_abs16_l() {
        let mut cpu = Cpu::new();
        cpu.write_abs16_l(0xff10, 0x0f0fff0f).await.unwrap();
        assert_eq!(
            (cpu.read_abs24_w(0xffff10).await.unwrap() as u32) << 16
                | cpu.read_abs24_w(0xffff12).await.unwrap() as u32,
            0x0f0fff0f
        );
    }

    // アドレスの最上位ビットが1のときのみ
    #[tokio::test]
    async fn test_read_abs16_l() {
        let mut cpu = Cpu::new();
        cpu.write_abs24_w(0xffff00, 0x0f0f).await.unwrap();
        cpu.write_abs24_w(0xffff02, 0xff0f).await.unwrap();
        assert_eq!(cpu.read_abs16_l(0xff00).await.unwrap(), 0x0f0fff0f);
    }

    #[tokio::test]
    async fn test_write_abs24_l() {
        let mut cpu = Cpu::new();
        cpu.write_abs24_l(0xffff10, 0x0f0fff0f).await.unwrap();
        assert_eq!(
            (cpu.read_abs24_w(0xffff10).await.unwrap() as u32) << 16
                | cpu.read_abs24_w(0xffff12).await.unwrap() as u32,
            0x0f0fff0f
        );
    }

    #[tokio::test]
    async fn test_read_abs24_l() {
        let mut cpu = Cpu::new();
        cpu.write_abs24_w(0xffff00, 0x0f0f).await.unwrap();
        cpu.write_abs24_w(0xffff02, 0xff0f).await.unwrap();
        assert_eq!(cpu.read_abs24_l(0xffff00).await.unwrap(), 0x0f0fff0f);
    }
}
