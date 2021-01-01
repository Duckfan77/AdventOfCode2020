use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let mut pairs: HashMap<String, HashSet<String>> = HashMap::new();

    //parse out list of allergens, to make the hashsets to match to them, and initial population of sets
    for line in text.lines() {
        let (ing, algs) = line.split_at(line.find("(").unwrap());
        let algs = &algs[9..algs.len()-1];

        for allergen in algs.trim().split(",") {
            let allergen = allergen.trim();
            //create the hashset if it doesn't exist. If it *does* exist, remove all elements not in the new item
            if !pairs.contains_key(allergen) {
                pairs.insert(String::from(allergen), ing.split(" ").map(|x| String::from(x)).collect::<HashSet<String>>());
            } else {
                pairs.get_mut(allergen).unwrap().retain(|i| ing.split(" ").map(|x| String::from(x)).collect::<Vec<String>>().contains(i));
            }
        }
    }

    //find ingredients that are definitely not allergens
    let mut clean: Vec<String> = Vec::new();
    for line in text.lines() {
        let (ing, _) = line.split_at(line.find("(").unwrap());
        for i in ing.split(" ") {
            let mut add = true;
            for v in pairs.values() {
                if v.contains(i) {
                    add = false;
                    break
                }
            }
            if add {
                clean.push(String::from(i));
            }
        }
    }

    println!("{:?}\n total number: {}", clean, clean.len());

    Ok(())
}