use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let (dep, rts) = text.split_at(text.find("\n").unwrap());

    let dep = dep.parse::<u64>().unwrap_or(0);
    let rts = rts.trim();

    let mut rt = 0;
    let mut wait = u64::MAX;

    for time in rts.split(",") {
        if time == "x" {
            continue;
        }

        let time = time.parse::<u64>().unwrap();
        let d = time - (dep%time); //calculate number of minutes you'll wait
        if d < wait {
            wait = d;
            rt = time;
        }
    }

    println!("rt: {}, wait: {}, ans: {}", rt, wait, rt*wait);

    Ok(())
}