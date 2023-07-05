mod bus;
mod cpu;
mod elf;
mod memory;
mod setting;

use clap::Parser;

use crate::cpu::Cpu;
use crate::memory::MEMORY_SIZE;

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

    /* // test code
    let cpu_emu_share = cpu.emu_share_values.clone();
    tokio::spawn(async move {
        loop {
            println!("{}", cpu_emu_share.lock().await.pc)
        }
    });
     */

    elf::load(args.elf, &mut cpu).await;
    // print_memory(&cpu.bus.memory);

    cpu.run().await.unwrap();
}
