mod bus;
mod cpu;
mod elf;
mod memory;
mod setting;

use clap::Parser;

use crate::cpu::Cpu;

#[derive(Parser)]
struct Args {
    /// path of the elf file to execute
    #[arg(short, long)]
    elf: String,

    /// Print executed opcode
    #[arg(long)]
    debug_instruction: Option<bool>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if let Some(v) = args.debug_instruction {
        *setting::ENABLE_PRINT_OPCODE.write().unwrap() = v;
    }

    let mut cpu = Cpu::new();

    elf::load(args.elf, &mut cpu);
    // print_memory(&cpu.bus.memory);

    cpu.run().await.unwrap();
}
