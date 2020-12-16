use std::fs::File;
use std::io::prelude::*;

struct Field {
    pub name: String,
    pub lower1: i32,
    pub lower2: i32,
    pub higher1: i32,
    pub higher2: i32,
    pub matches: Vec<bool>,
    pub nmatches: i32,
    pub m: usize,
}

impl Field {
    #[allow(dead_code)]
    pub fn new() -> Field {
        Field{
            name: String::from(""),
            lower1: 0,
            lower2: 0,
            higher1: 0,
            higher2: 0,
            matches: Vec::new(),
            nmatches: 0,
            m: 0,
        }
    }

    pub fn new_pop(name: String, l1: i32, l2: i32, h1: i32, h2: i32) -> Field {
        Field{
            name: name,
            lower1: l1,
            lower2: l2,
            higher1: h1,
            higher2: h2,
            matches: Vec::new(),
            nmatches: 0,
            m: 0,
        }
    }

    pub fn valid(&self, val: i32) -> bool {
        (val >= self.lower1 && val <= self.higher1) || (val >= self.lower2 && val <= self.higher2)
    }

    pub fn parse_rule(rule: String) -> Field {
        //println!("Parsing rule: {}", rule);
        let (name, r) = rule.split_at(rule.find(":").unwrap());
        let r = r[1..].trim();//remove the ":"
        let (r1, r2) = r.split_at(r.find("or").unwrap());
        let r1 = r1.trim();
        let r2 = r2[2..].trim();//remove the "or"

        let (l1, h1) = r1.split_at(r1.find("-").unwrap());
        let l1 = l1.parse::<i32>().unwrap();
        let h1 = h1[1..].parse::<i32>().unwrap();

        let (l2, h2) = r2.split_at(r2.find("-").unwrap());
        let l2 = l2.parse::<i32>().unwrap();
        let h2 = h2[1..].parse::<i32>().unwrap();

        Field::new_pop(String::from(name), l1, l2, h1, h2)
    }

    pub fn populate_matches(&mut self, num: i32) -> () {
        for _ in 0..num {
            self.matches.push(true);
        }
        self.nmatches = num;
    }

    pub fn valid_remove(&mut self, val: i32, idx: usize) -> bool {
        if self.valid(val) {
            true
        } else {
            if self.matches[idx] {
                self.nmatches -= 1;
            }
            self.matches[idx] = false;
            if self.nmatches == 1 {
                for (i, v) in self.matches.iter().enumerate() {
                    if *v {
                        self.m = i;
                        self.nmatches = 0;
                        break;
                    }
                }
            }
            false
        }
    }
}


fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let mut fields: Vec<Field> = Vec::new();

    //Will always have 3 blocks separated by "\n\n" characters
    let mut blocks = text.split("\n\n");
    let rules = blocks.next().unwrap();
    let (_, own) = blocks.next().unwrap().split_at(13);    //hardcoded splits to remove labels
    let (_, others) = blocks.next().unwrap().split_at(16);  //hardcoded splits to remove labels

    for rule in rules.lines() {
        fields.push(Field::parse_rule(String::from(rule)));
    }

    let count = fields.len() as i32;
    for field in fields.iter_mut() {
        field.populate_matches(count);
    }

    let mut nnotfound = count;
    let mut v: Vec<usize> = Vec::new();
    let mut vtemp: Vec<usize> = Vec::new();
    //for each ticket

    for ticket in others.lines() {
        //check every value in the ticket
        for (i, val) in ticket.split(",").enumerate() {
            //check each value against every rule to see if it passes at least one
            let v2 = val.parse::<i32>().unwrap();
            for (j, field) in fields.iter_mut().enumerate() {
                if field.nmatches > 1 {//skip fields that have already been found
                    field.valid_remove(v2, i);
                    if field.nmatches == 0 {
                        nnotfound -= 1;
                        v.push(field.m);
                        println!("Rule {} idx {} must match with slot {}", field.name, j, field.m)
                    }
                }
            }
        }
    }

    while v.len() > 0 {
        for m in v.drain(..) {
            for (j, field) in fields.iter_mut().enumerate() {
                if field.nmatches > 1{
                    field.valid_remove(-1, m);
                    if field.nmatches == 0 {
                        nnotfound -= 1;
                        vtemp.push(field.m);
                        println!("Rule {} idx {} must match with slot {}", field.name, j, field.m)
                    }
                }
            }
        }
        v.append(&mut vtemp);
        /*for rule in fields.iter() {
            println!("Rule {} has {} options, match: {}, {:?}", rule.name, rule.nmatches, rule.m, rule.matches);
        }*/
        println!("End of cycle: nnotfound: {}\n", nnotfound);
    }



    let mut ownv: Vec<i32> = Vec::new();
    for val in own.split(",") {
        ownv.push(val.parse::<i32>().unwrap());
    }

    let mut total: u64 = 1;
    for field in fields[..6].iter() {//first 6 contain departure, just check those
        for (i, v) in field.matches.iter().enumerate() {
            if *v {
                total *= ownv[i] as u64;
                break;
            }
        }
    }

    println!("Total product: {}", total);

    Ok(())
}