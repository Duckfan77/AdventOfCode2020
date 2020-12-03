use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let mut index : usize = 0;
    let mut n = 0;

    //iterate over all lines, checking for collision with trees
    for line in text.lines() {
        if line[index..].chars().next().unwrap() == '#' {
            n+=1;
        }
        index += 3;
        index %= line.len();
    }

    println!("Encountered {} trees", n);

    Ok(())
}
