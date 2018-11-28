extern crate melon;
extern crate rand;
extern crate minifb;

mod system;

use system::MatrixSystem;
use melon::{VM, Program, ProgramBuilder, Instruction};

fn main() {
    let mut matrix = MatrixSystem::new();

    let program = ProgramBuilder::new("org.bakervm.tmod".into()).instructions(vec![
        Instruction::SysCall(1),
        Instruction::Jmp(false, 1),
        ]).gen();

    VM::default().exec(&program, &mut matrix).unwrap();
}
