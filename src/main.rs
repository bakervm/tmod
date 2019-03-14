extern crate melon;
extern crate minifb;
extern crate rand;

mod system;

use melon::{Debugger, Instruction, IntegerType, Program, ProgramBuilder, VM};
use system::MatrixSystem;

fn main() {
    let rom_data = include_bytes!("../stock/target/stock.rom");

    let program = Program::from_slice(rom_data).expect("unable to load rom");

    VM::default().exec(&program, &mut MatrixSystem::new()).unwrap_or_else(|e| panic!("Error: {}", e));
}
