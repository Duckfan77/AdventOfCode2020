use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::hash_map::Iter;

const ONES: u64 = 0xFFFFFFFFFFFFFFFF;
const ZEROS: u64 = 0;
const TRAIL_ONE: u64 = 1;
const TRAIL_ZERO: u64 = 0xFFFFFFFFFFFFFFFE;

fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let mut state = ProgramState::new();
    for line in text.lines() {
        let (pre, post) = line.split_at(line.find("=").unwrap());
        let pre = pre.trim();
        let post = post[1..].trim();//remove the = character

        if pre == "mask" { //mask set
            println!("Setting Mask to {}", post);
            state.set_mask(String::from(post));
        }else { //memory set
            //println!("val: {}, loc: {}", post, pre);
            let val = post.parse::<u64>().unwrap();
            let loc = pre[pre.find("[").unwrap()+1..pre.find("]").unwrap()].parse::<u64>().unwrap();
            println!("Assigning {} to {} premap", val, loc);
            state.add_val(loc, val);
        }
    }

    let mut k = 0;
    for i in state.get_mem_iter() {
        let (_, j) = i;
        k += j;
    }

    println!("Sum is {}", k);

    Ok(())
}

struct ProgramState {
    pub mem: HashMap<u64, u64>,
    pub mask: String,
    pub mask1: u64,
    pub mask0: u64,
}

impl ProgramState {
    pub fn new() -> ProgramState {
        ProgramState{
            mem: HashMap::new(),
            mask: String::from(""),
            mask1: ZEROS,
            mask0: ONES,
        }
    }

    pub fn add_val(&mut self, loc: u64, val: u64) -> () {
        let val = val & self.mask0; //mask in the 0 overwrites
        let val = val | self.mask1; //mask in the 1 overwrites
        self.mem.insert(loc, val);
        println!("Wrote {} to {} postmask", val, loc);
    }

    pub fn get_mem_iter(&self) -> Iter<u64, u64> {
        self.mem.iter()
    }

    pub fn set_mask(&mut self, mask: String) -> () {
        //init values
        self.mask=mask;
        self.mask1 = ZEROS;
        self.mask0 = ONES;

        for c in self.mask.chars() {
            match c {
                '1' => {
                    //Insert active character for masking in 1s, 1
                    self.mask1 <<= 1;
                    self.mask1 |= TRAIL_ONE;

                    //Insert silent character for masking in 0s, 1
                    self.mask0 <<= 1;
                    self.mask0 |= TRAIL_ONE;
                }

                '0' => {
                    //Insert silent character for masking in 1s, 0
                    self.mask1 <<= 1;
                    self.mask1 &= TRAIL_ZERO;

                    //Insert active character for masking in 0s, 0
                    self.mask0 <<= 1;
                    self.mask0 &= TRAIL_ZERO;
                }

                'X' => {
                    //Insert silent character for masking in 1s, 0
                    self.mask1 <<= 1;
                    self.mask1 &= TRAIL_ZERO;

                    //Insert silent character for masking in 0s, 1
                    self.mask0 <<= 1;
                    self.mask0 |= TRAIL_ONE;
                }

                 _  => {
                     panic!("Unexpected character {} in mask {}", c, self.mask);
                 }
            }
        }
    }
}