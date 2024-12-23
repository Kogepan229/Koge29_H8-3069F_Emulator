mod bus;
mod cpu;
mod elf;
mod ioport;
mod memory;
mod modules;
mod registers;
mod setting;
mod socket;

use clap::Parser;
use log::error;

use crate::cpu::Cpu;

#[derive(Parser)]
#[command(version)]
struct Args {
    /// path of the elf file to execute
    #[arg(short, long)]
    elf: String,

    #[arg(short, long, default_value = "")]
    args: String,

    /// log level (error, warn, info, debug, trace, off)
    #[arg(short = 'l', long, default_value = "info")]
    log: String,

    /// Print executed opcode
    #[arg(short = 'i', long)]
    print_instruction: bool,

    #[arg(short = 'm', long)]
    print_messages: bool,

    #[arg(short, long)]
    socket: bool,

    /// Wait for a start message from the socket
    #[arg(short = 'w', long)]
    wait_start: bool,

    #[arg(short, long, default_value = "12345")]
    port: u16,
}

fn main() {
    let args = Args::parse();

    init_logger(args.log);

    *setting::ENABLE_PRINT_OPCODE.write().unwrap() = args.print_instruction;
    *setting::ENABLE_PRINT_MESSAGES.write().unwrap() = args.print_messages;
    *setting::ENABLE_WAIT_START.write().unwrap() = args.socket && args.wait_start;

    let mut cpu = Cpu::new();

    #[cfg(not(test))]
    if args.socket {
        cpu.connect_socket(&format!("127.0.0.1:{}", args.port)).unwrap();
    }

    elf::load(args.elf, &mut cpu, args.args);

    cpu.run().unwrap();
}

fn _init_logger(level: &str) {
    use std::io::Write;
    env_logger::Builder::from_env(env_logger::Env::new().default_filter_or(level))
        .format(|buf, record| {
            let level_style = buf.default_level_style(record.level());
            let level_str = format!("{}{}{:#}", level_style, record.level(), level_style);

            writeln!(buf, "[{}] {}", level_str, record.args())
        })
        .init();
}

fn init_logger(arg_log_level: String) {
    match arg_log_level.to_lowercase().as_str() {
        "error" | "warn" | "info" | "debug" | "trace" | "off" => _init_logger(&arg_log_level),
        _ => {
            _init_logger("info");
            error!("Invalid log_level: {}", arg_log_level);
        }
    }
}
