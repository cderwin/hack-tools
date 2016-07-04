#![allow(unused_parens)]

mod state;
mod vm;
mod assembler;

use std::env;
use std::fs::File;
use std::io::Read;
use std::process::exit;

use state::{Register,State};


fn main(){
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];
    let mut instructions = load_instructions(fname);

    // main loop
    let mut state = State::new();
    state.load_instructions(&mut instructions);
    loop {
        if !update(&mut state) {
            break;
        }
    }
}


fn update(state: &mut State) -> bool {
    let instruction = state.fetch_instruction();
    if (instruction & 0x8000) == 0 {
        state.write_reg(Register::A, instruction);
        state.tick(false);
        return true;
    }

    let comp = (instruction & 0x1fc0) >> 6; // bits 3 - 9 (0x1fc0 = 0b0001 1111 1100 0000) of instruction
    let dest = (instruction & 0x0038) >> 3; // bits 10 - 12 (0x0038 = 0b0000 0000 0011 1000) of instruction
    let jmp = (instruction & 0x0007); // bits 13 - 15 (0x0007 = 0b0000 0000 0000 0111) of instruction

    // Perform calculation

    let y_register = if (comp & 0x40) == 0 { Register::A } else { Register::M };
    let out = alu_computation(state.read_reg(Register::D), state.read_reg(y_register), comp);

    // save result in registers

    if (dest & 0x4) == 1 {
        state.write_reg(Register::A, out);
    }

    if (dest & 0x2) == 1 {
        state.write_reg(Register::M, out);
    }

    if (dest & 0x1) == 1 {
        state.write_reg(Register::D, out);
    }


    // update program counter
    let jump: bool = match jmp {
        0b000 => false,
        0b001 => (out as i16) > 0,
        0b010 => (out as i16) == 0,
        0b011 => (out as i16) >= 0,
        0b100 => (out as i16) < 0,
        0b101 => (out as i16) != 0,
        0b110 => (out as i16) <= 0,
        0b111 => true,
        _ => {
            println!("Invalid `jmp` instruction; assertion failed");
            exit(-1);
        }
    };

    state.tick(jump);

    true
}



// ALU implementation
fn alu_computation(x: u16, y: u16, comp: u16) -> u16 {
    let mut x_in = x;
    let mut y_in = y;

    // zx -- set x to zero
    if (comp & 0x20) != 0 {
        x_in = 0;
    }

    // nx -- invert x
    if (comp & 0x10) != 0 {
        x_in = !x_in;
    }

    // zy -- set y to zero
    if (comp & 0x08) != 0 {
        y_in = 0;
    }

    // ny -- invert y
    if (comp & 0x04) != 0 {
        y_in = !y_in;
    }

    // f -- add if true (1), and if false (0)
    let mut out: u16 = if (comp & 0x02) == 0 { x_in & y_in } else { x_in.wrapping_add(y_in) };

    // no -- invert output
    if (comp & 0x01) != 0 {
        out = !out;
    }

    out
}



fn load_instructions(fname: &String) -> Vec<u16> {

    let mut file = match File::open(&fname) {
        Ok(f) => f,
        Err(_) => {
            println!("Could not open file {}", fname);
            exit(1);
        }
    };


    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(a) => a,
        Err(_) => {
            println!("Could not read contents of {}", &fname);
            exit(1);
        }
    };


    contents.split("\n")
    .map(|line| {

        let value = match u16::from_str_radix(&line, 2) {
            Ok(o) => o,
            Err(_) => {
                println!("{} could not be parsed as an instruction", &line);
                exit(1)
            }
        };

        value
    }).collect()
}
