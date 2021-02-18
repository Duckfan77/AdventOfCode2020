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

    //Iterate over the Vec, getting three distinct elements, to check if meet the criteria of summing to 2020
    for (i, el1) in v.iter().enumerate() {
        for (j, el2) in v[i+1..].iter().enumerate() {
            for el3 in v[j+1..].iter() {
                if el1 + el2 + el3 ==2020 {
                    println!("{}", el1 * el2 * el3);
                    return Ok(())
                }
            }
        }
    }

    Ok(())
}
