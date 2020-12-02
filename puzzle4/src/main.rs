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
        for c2 in line[colon+1..].chars() {
            if c2 == c {
                ccount+=1;
            }
        }
        println!("ccount: {}, maxc: {}, minc: {}", ccount, maxc, minc);
        if ccount <= maxc && ccount >= minc {
            n +=1;
        }
    }
    println!("{} passwords work, out of {}", n, x);

    Ok(())
}
