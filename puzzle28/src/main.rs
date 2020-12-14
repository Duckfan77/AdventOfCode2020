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
    pub masks: Vec<(u64, u64)>,//(mask1, mask0)
}

impl ProgramState {
    pub fn new() -> ProgramState {
        ProgramState{
            mem: HashMap::new(),
            mask: String::from(""),
            masks: Vec::new(),
        }
    }

    pub fn add_val(&mut self, loc: u64, val: u64) -> () {
        for (mask1, mask0) in self.masks.iter() {
            let loc = loc & mask0;
            let loc = loc | mask1;
            self.mem.insert(loc, val);
            println!("Wrote {} to {} postmask", val, loc);
        }
    }

    pub fn get_mem_iter(&self) -> Iter<u64, u64> {
        self.mem.iter()
    }

    pub fn set_mask(&mut self, mask: String) -> () {
        //init values
        self.mask=mask;
        self.masks.clear();

        //get count of Xs, to get number of ways to arrange floating bits
        let mut xcount=0;
        for c in self.mask.chars() {
            if c == 'X' {
                xcount += 1;
            }
        }

        //create a mask for each arrangement of floating bits
        for i in 0..(2u64.pow(xcount)){
            let mut worki = i;
            let mut mask1 = ZEROS;
            let mut mask0 = ONES;
            for c in self.mask.chars() {
                match c {
                    '1' => {
                        //Insert active character for masking in 1s, 1
                        mask1 <<= 1;
                        mask1 |= TRAIL_ONE;

                        //Insert silent character for masking in 0s, 1
                        mask0 <<= 1;
                        mask0 |= TRAIL_ONE;
                    }

                    '0' => {
                        //Insert silent character for masking in 1s, 0
                        mask1 <<= 1;
                        mask1 &= TRAIL_ZERO;

                        //Insert silent character for masking in 0s, 0
                        mask0 <<= 1;
                        mask0 |= TRAIL_ONE;
                    }

                    'X' => {//insert based on least significant bit of i, and then shift i
                        match worki%2 {
                            0 => {//mask in a 0 this time
                                mask1 <<= 1;
                                mask1 &= TRAIL_ZERO;

                                mask0 <<= 1;
                                mask0 &= TRAIL_ZERO;
                            }

                            1=> {//mask in 1 this time
                                //Insert active character for masking in 1s, 1
                                mask1 <<= 1;
                                mask1 |= TRAIL_ONE;

                                //Insert silent character for masking in 0s, 1
                                mask0 <<= 1;
                                mask0 |= TRAIL_ONE;
                            }

                            _ => {
                                panic!("Unexpected value in modulo 2 of i {}", i%2);
                            }
                        }
                        worki >>= 1;
                    }

                    _  => {
                        panic!("Unexpected character {} in mask {}", c, self.mask);
                    }
                }
            }
            self.masks.push((mask1, mask0));
        }
    }
}