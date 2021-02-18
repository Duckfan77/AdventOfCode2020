use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn num_yes(block : &str) -> usize {
    let mut set: HashSet<char> = HashSet::with_capacity(10);
    let mut v : Vec<char> = Vec::new();

    //populate set with all characters, to be whittled down
    for c in "abcdefghijklmnopqrstuvwxyz".chars(){
        set.insert(c);
    }

    //iterate over lines
    for line in block.lines() {
        v.clear();
        //get chars in line
        for c in line.chars() {
            //insert the character
            v.push(c);
        }
        //retain only the characters that are in this line in the set
        set.retain(|c| v.contains(c));
    }

    return set.len();
}

fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let mut n=0;

    //split at blank lines to get blocks
    for block in text.split_terminator("\n\n") {
        n += num_yes(&block);
    }

    println!("There are {} yes answers", n);
    Ok(())
}
