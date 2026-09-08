use std::io::stdin;

use piku::{assembler::Assembler, cpu::CPU, encoder::encode};

fn main() {
    let input = get_input();
    
    let mut assembler = Assembler::new(input);
    let items = match assembler.process() {
        Ok(i) => i,
        Err(e) => {
            println!("An error ocurred while parsing the assembly: {e}");
            return;
        },
    };
    println!("{:?}", items);

    let program = match encode(items) {
        Ok(v) => v,
        Err(e) => {
            println!("An error occured during enocding: {e}");
            return;
        },
    };
    println!("{:?}", program);

    let mut cpu = CPU::new();
    cpu.load(program);
    if let Err(e) = cpu.run(false) {
        println!("{e}");
        return;
    };
    println!("{:?}", cpu.reg);
}

fn get_input() -> String {
    let mut s = String::new();
    if matches!(stdin().read_line(&mut s), Err(_)) {
        s = get_input();
    }
    s.trim().to_string()
}
