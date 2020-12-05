use std::fs::File;
use std::io::prelude::*;

fn get_val(line : &str) -> u32 {
    let mut _top = 128;
    let mut bot = 0;
    let mut size = 64;

    let mut l = 0;
    let mut _r = 8;
    let mut size2 = 4;

    for (i, c) in line.chars().enumerate() {
        match i {
            0|1|2|3|4|5|6 => {
                match c {
                    'F' => {
                        _top -= size;
                        size /=2;
                    },
                    'B' => {
                        bot += size;
                        size /= 2;
                    },
                    _ => {
                        println!("Got bad character {} when looking for F or B", c);
                    }
                }
            },
            7|8|9 => {
                match c {
                    'L' => {
                        _r -= size2;
                        size2 /=2;
                    },
                    'R' => {
                        l += size2;
                        size2 /= 2;
                    },
                    _ => {
                        println!("Got bad character {} when looking for L or R", c);
                    }
                }
            },
            _ => {

            }
        }
    }

    //println!("{} Row, {} Col, {} ID, {} line", bot, l, bot*8+l, line);

    return bot*8+l;
}

fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let mut v : Vec<u32> = Vec::new();

    //split at blank lines to get blocks
    for line in text.lines() {
        let new = get_val(&line);
        v.push(new);
    }

    //sort v
    v.sort_unstable();
    let mut last = 0;
    for i in v {
        //println!("{}", i);
        if i-last==2 {
            println!("{} is an open seat ID", last + 1);
            last += 1;
            break;
        }
        last = i;
    }

    println!("{} is my seat ID", last);
    Ok(())
}
