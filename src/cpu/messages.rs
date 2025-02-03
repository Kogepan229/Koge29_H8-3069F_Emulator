use crate::{bus::Bus, setting};

use super::Cpu;
use anyhow::Result;

impl Cpu {
    #[cfg(not(test))]
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

    #[cfg(not(test))]
    pub fn parse_ioport(&mut self, list: Vec<&str>) {
        if list.len() != 3 {
            return;
        }
        let port_result = u8::from_str_radix(&list[1], 16);
        let value_result = u8::from_str_radix(&list[2], 16);
        if let Ok(port) = port_result {
            if let Ok(value) = value_result {
                self.bus.write_port(port, value);
            }
        }
    }

    pub fn send_message(&mut self, message: &String) -> Result<()> {
        #[cfg(not(test))]
        if let Some(socket) = &self.socket {
            socket.send_message(message)?;
        }
        if *setting::ENABLE_PRINT_MESSAGES.read().unwrap() {
            println!("msg: {}", message);
        }
        Ok(())
    }

    pub fn send_ready_message(&mut self) -> Result<()> {
        self.send_message(&"ready".to_string())?;
        Ok(())
    }

    pub fn send_sync_message(&mut self) -> Result<()> {
        self.send_message(&format!("sync:{}", self.state_sum))?;
        Ok(())
    }

    pub fn send_stdout_message(&mut self, string: &String) -> Result<()> {
        self.send_message(&format!("stdout:{}", string))?;
        Ok(())
    }
}

impl Bus {
    pub fn send_message(&mut self, message: &String) -> Result<()> {
        if let Some(tx) = &self.message_tx {
            tx.send(message.clone())?;
        }
        if *setting::ENABLE_PRINT_MESSAGES.read().unwrap() {
            println!("msg: {}", message);
        }
        Ok(())
    }

    pub fn send_io_port_value(&mut self, port: u8, value: u8) -> Result<()> {
        let str = format!("ioport:{:x}:{:x}:{}", port, value, self.cpu_state_sum);
        self.send_message(&str)?;
        Ok(())
    }

    pub fn send_addr_value_u8(&mut self, addr: u32, value: u8) -> Result<()> {
        let str = format!("u8:{:x}:{:x}", addr, value);
        self.send_message(&str)?;
        Ok(())
    }
}
