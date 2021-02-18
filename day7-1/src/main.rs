use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let mut total : HashSet<&str> = HashSet::new();
    let mut looking : HashSet<&str> = HashSet::new();
    let mut new : HashSet<&str> = HashSet::new();
    let mut found = true;
    looking.insert("shiny gold");

    while found {//find the bags that contain the new bag types to look for, so long as new ones are found
        found = false;
        new.clear();
        for line in text.lines() {
            let (name, rule) = line.split_at(line.find("contain").unwrap() - 6 as usize);
            for i in looking.iter() {
                if rule.contains(i) {
                    if total.insert(name) {
                        new.insert(name);
                        found = true;
                    }
                }
            }
        }
        //copy the new items into the looking set
        looking.clear();
        for i in new.iter(){
            looking.insert(i);
        }
    }

    println!("There are {} options for top level bags", total.len());

    Ok(())
}
