use super::Cpu;
use anyhow::Result;

impl Cpu {
    pub fn parse_u8(&mut self, list: Vec<&str>) -> Result<()> {
        if list.len() != 3 {
            return Ok(());
        }
        let addr_result = u32::from_str_radix(&list[1], 16);
        let value_result = u8::from_str_radix(&list[2], 16);
        if let Ok(addr) = addr_result {
            if let Ok(value) = value_result {
                if self.bus.write(addr, value).is_err() {
                    log::warn!("Received invalid u8 addr: 0x{:x}, value: 0x{:x}", addr, value);
                }
            }
        }
        Ok(())
    }
}
