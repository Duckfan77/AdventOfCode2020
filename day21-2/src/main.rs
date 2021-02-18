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
                pairs.insert(String::from(allergen), ing.split_terminator(" ").map(|x| String::from(x)).collect::<HashSet<String>>());
            } else {
                pairs.get_mut(allergen).unwrap().retain(|i| ing.split_terminator(" ").map(|x| String::from(x)).collect::<Vec<String>>().contains(i));
            }
        }
    }

    //same solver code from puzzle 32 (day16-2), adapted to fit current datastructures
    let mut v: Vec<String> = pairs.values().filter(|x| x.len()==1).map(|x| String::from(x.iter().next().unwrap())).collect();
    let mut vtemp: Vec<String> = Vec::new();

    //println!("v: {:?}\n", v);

    while v.len() > 0 {
        vtemp.clear();
        for used in v.drain(..) {
            //println!("Examining {}", used);
            for (_a, list) in pairs.iter_mut() {
                if list.len() > 1 {
                    list.remove(&used);
                    //println!("Removing {} from {} list", used, _a);
                    if list.len() == 1 {
                        vtemp.push(String::from(list.iter().next().unwrap()));
                        //println!("Adding {} to vtemp", list.iter().next().unwrap());
                    }
                }
            }
        }
        v.append(&mut vtemp);
        println!("End of Cycle, v: {:?}", v);
    }

    let mut  out = String::new();
    let mut sorted_keys = pairs.keys().map(|x| String::from(x)).collect::<Vec<String>>();
    sorted_keys.sort_unstable();
    for allergen in sorted_keys.iter() {
        out.push_str(pairs.get(allergen).unwrap().iter().next().unwrap());
        out.push(',');
    }

    println!("output string:\n{}", out);

    Ok(())
}