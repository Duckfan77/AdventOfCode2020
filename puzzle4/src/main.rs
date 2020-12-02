use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let mut n = 0;
    let mut x = 0;
    for line in text.lines() {
        x += 1;
        let min = match line.find("-") {
            None => continue,
            Some(f) => f,
        };
        let minc = line[..min].parse::<i32>().unwrap_or(-1);
        let max = match line.find(" ") {
            None => continue,
            Some(f) => f,
        };
        let maxc = line[min+1..max].parse::<i32>().unwrap_or(-1);
        let colon = match line[max..].find(":") {
            None=>continue,
            Some(f) => f+max,
        };
        let c : char = match line[colon-1..colon].chars().next() {
            None => continue,
            Some(f) => f,
        };
        let mut ccount=0;

        println!("char1i: {}, char2i: {}, char: {}, line: {}", minc, maxc, c, &line[colon+1..]);
        println!("char1: {}, char2: {}", &line[colon + 1 + minc as usize ..], &line[colon + 1 + maxc as usize..]);
        //check char 1
        if c == match line[colon +1+ minc as usize..].chars().next() {
            None=> continue,
            Some(f) => f,
        } {
            println!("char 1 match");
            ccount+=1;
        }

        if c == match line[colon+1+maxc as usize ..].chars().next() {
            None => continue,
            Some(f) => f,
        } {
            println!("char 2 match");
            ccount+=1;
        }

        if ccount ==1 {
            println!("Valid");
            n +=1;
        }else{
            println!("Invalid");
        }
    }
    println!("{} passwords work, out of {}", n, x);

    Ok(())
}
