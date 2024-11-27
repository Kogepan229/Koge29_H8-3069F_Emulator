mod bus;
mod cpu;
mod elf;
mod memory;
mod modules;
mod registers;
mod setting;
mod socket;

use clap::Parser;

use crate::cpu::Cpu;

#[derive(Parser)]
#[command(version)]
struct Args {
    /// path of the elf file to execute
    #[arg(short, long)]
    elf: String,

    #[arg(short, long, default_value = "")]
    args: String,

    /// Print executed opcode
    #[arg(short = 'i', long)]
    print_instruction: bool,

    #[arg(short = 'm', long)]
    print_messages: bool,

    #[arg(short, long)]
    disable_socket: bool,

    #[arg(short = 'r', long)]
    print_ready: bool,

    #[arg(short, long, default_value = "12345")]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    *setting::ENABLE_PRINT_OPCODE.write().unwrap() = args.print_instruction;

    *setting::ENABLE_PRINT_MESSAGES.write().unwrap() = args.print_messages;

    *setting::ENABLE_PRINT_READY.write().unwrap() = args.print_ready;

    let mut cpu = Cpu::new();

    if !args.disable_socket {
        socket::listen(format!("127.0.0.1:{}", args.port)).await.unwrap();
    }

    elf::load(args.elf, &mut cpu, args.args);

    cpu.run().await.unwrap();
}
