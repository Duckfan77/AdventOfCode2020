use std::fs::File;
use std::io::prelude::*;

#[derive(Eq, PartialEq)]
enum Opcode {
    Jmp,
    Acc,
    Nop,
}

#[derive(Eq, PartialEq)]
struct Instr {
    pub op: Opcode,
    pub val: i32,
    pub touched: bool,
}

impl Instr {
    pub fn new(op: Opcode, val: i32) -> Instr {
        Instr{
            op: op,
            val: val,
            touched: false,
        }
    }
}

fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    //populate vector of instr based on the information in the input file
    let mut v: Vec<Instr> = Vec::new();
    for line in text.lines(){
        let (op, val) = line.split_at(3);
        let val = val.trim().parse::<i32>().unwrap();
        match op {
            "jmp" => v.push(Instr::new(Opcode::Jmp, val)),
            "acc" => v.push(Instr::new(Opcode::Acc, val)),
            "nop" => v.push(Instr::new(Opcode::Nop, val)),
            &_    => panic!("{} is not a valid opcode!", op),
        };
    }

    let mut save=0;

    for ptr in 0..v.len() {
        //Change the nop or jmp instruction under the pointer, skip if acc
        //println!("Adjusting index {}", ptr);
        match v[ptr].op {
            Opcode::Jmp => v[ptr].op = Opcode::Nop,
            Opcode::Nop => v[ptr].op = Opcode::Jmp,
            Opcode::Acc => {
                //println!("Is ACC, skip");
                continue;
            },
        };

        let mut acc = 0;
        let mut i = 0;
        let mut term = false;
        //Follow instructions
        loop {
            //check for termination
            if i >= v.len() {
                //println!("TERMINATES");
                term = true;
                break;
            }

            //stop at start of infinite loop
            if v[i].touched {break;}

            //mark instruction as visited
            v[i].touched = true;

            //execute each instruction
            match v[i].op {
                //change instruction pointer by val on jmp instructions
                Opcode::Jmp => {
                    i = (i as i32 + v[i].val) as usize;
                },

                //change global accumulator by val, then go to next instruction on acc
                Opcode::Acc => {
                    acc += v[i].val;
                    i += 1;
                },

                //Just go to next instruction on nops
                Opcode::Nop => {
                    i += 1;
                }
            }
        }

        //program terminates
        if term {
            save = acc;
            break;
        }

        //clean up touched not reseting
        for instr in v.iter_mut() {
            instr.touched = false;
        }

        //change the instruction back, it doesn't terminate
        match v[ptr].op {
            Opcode::Jmp => v[ptr].op = Opcode::Nop,
            Opcode::Nop => v[ptr].op = Opcode::Jmp,
            Opcode::Acc => (),
        };
    }

    println!("Accumulator is {} immediately prior to termination", save);

    Ok(())
}