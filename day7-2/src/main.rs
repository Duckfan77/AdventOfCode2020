use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

//Recursively calculates the number of bags within the bag, counting itself
fn bag_num(name: String, map: &HashMap<String, Vec<(i32, String)>>) -> i32 {
    let mut count = 1;

    println!("Examining {}", name);

    let list = map.get(&name).unwrap();
    for (n, bag) in list {
        if *n == 0 {//empty list, just break out
            break;
        } else {
            count += n * bag_num(String::from(bag), map);
        }
    }

    return count
}

fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let mut map: HashMap<String, Vec<(i32, String)>> = HashMap::new();

    for line in text.lines() {
        let (name, rule) = line.split_at(line.find(" contain ").unwrap());
        let (_, rule) = rule.split_at(9);

        //replace bags with bag in all strings for better matching, remove trailing .
        let name = name.replace("bags", "bag");
        let rule = rule.replace("bags", "bag").replace(".", "");

        //handle empty rules
        if rule=="no other bag" {
            map.insert(name, vec![(0, String::from(""))]);
        }else {//handle other rules
            let mut v:Vec<(i32, String)> = Vec::new();
            //split at each ", " to get number bag sets
            for subbag in rule.split(", ") {
                //split the subbag to get number and bag, add to vector
                let (n, bag) = subbag.split_at(1);
                let n = n.parse::<i32>().unwrap_or(0);
                let bag = bag.trim();
                v.push((n, String::from(bag)));

                //println!("{}, {}, {}", name, n, String::from(bag));
            }
            //println!("Inserting key {}", name);
            map.insert(name, v);
        }
    }
/*
    for (name, val) in (&map).iter() {
        println!("{}, {:?}", name, val);
    }*/

    let n = bag_num(String::from("shiny gold bag"), &map);

    println!("The shiny gold bag consists of {} bags", n-1);

    Ok(())
}
