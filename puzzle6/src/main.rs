use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let mut index : usize;
    let mut n;
    let mut x : u64 = 1;
    let slope = [1, 3, 5, 7];

    //not the most efficient, iterate over full file for each slope to check
    for s in slope.iter() {
        n = 0;
        index = 0;
        //iterate over all lines, checking for collision with trees
        for line in text.lines() {
            if line[index..].chars().next().unwrap() == '#' {
                n+=1;
            }
            index += s;
            index %= line.len();
        }
        println!("Encountered {} trees on slope R: {}, D: 1", n, s);
        x *= n;
    }

    //Check R1D2 slope
    let mut skip = false;

    n=0;
    index = 0;
    //iterate over all lines, checking for collision with trees
    for line in text.lines(){
        skip = !skip;
        if !skip {//skip line if skip is true, to get the down2 behavior
            continue;
        }
        if line[index..].chars().next().unwrap() == '#' {
            n+=1;
        }
        index += 1;
        index %= line.len();
    }
    println!("Encountered {} trees on slope R: 1, D: 2", n);
    x*=n;

    println!("product of all tree counts: {}", x);

    Ok(())
}
