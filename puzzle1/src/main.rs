use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    //create a Vec of i32 containing the elements of the input
    let mut v: Vec<i32> = Vec::new();
    for line in text.lines() {
        v.push(line.parse::<i32>().unwrap_or(0));
    }

    for (i, el1) in v.iter().enumerate() {
        for el2 in v[i+1..].iter() {
            if el1 + el2 == 2020 {
                println!("{}", el1 * el2);
                return Ok(())
            }
        }
    }

    Ok(())
}
