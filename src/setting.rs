use std::sync::RwLock;
#[cfg(debug_assertions)]
pub static ENABLE_PRINT_OPCODE: RwLock<bool> = RwLock::new(true);
#[cfg(not(debug_assertions))]
pub static ENABLE_PRINT_OPCODE: RwLock<bool> = RwLock::new(false);

pub static ENABLE_PRINT_MESSAGES: RwLock<bool> = RwLock::new(false);

pub static ENABLE_WAIT_START: RwLock<bool> = RwLock::new(false);
