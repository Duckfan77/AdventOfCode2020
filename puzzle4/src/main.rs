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
        //get min character count
        let min = match line.find("-") {
            None => continue,
            Some(f) => f,
        };
        let minc = line[..min].parse::<i32>().unwrap_or(-1);

        //get max character count
        let max = match line.find(" ") {
            None => continue,
            Some(f) => f,
        };
        let maxc = line[min+1..max].parse::<i32>().unwrap_or(-1);


        let colon = match line[max..].find(":") {
            None=>continue,
            Some(f) => f+max,
        };

        //get position of the colon
        let c : char = match line[colon-1..colon].chars().next() {
            None => continue,
            Some(f) => f,
        };

        let mut ccount=0;
        //check char 1, need to add 1 to account for the space
        if c == match line[colon +1+ minc as usize..].chars().next() {
            None=> continue,
            Some(f) => f,
        } {
            ccount+=1;
        }

        //Check char 2, need to add 1 to account for the space
        if c == match line[colon+1+maxc as usize ..].chars().next() {
            None => continue,
            Some(f) => f,
        } {
            ccount+=1;
        }

        //Check for valid number of characters
        if ccount ==1 {
            n +=1;
        }
    }
    println!("{} passwords work, out of {}", n, x);

    Ok(())
}
