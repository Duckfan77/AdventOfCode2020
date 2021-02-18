use std::fs::File;
use std::io::prelude::*;

struct Field {
    pub name: String,
    pub lower1: i32,
    pub lower2: i32,
    pub higher1: i32,
    pub higher2: i32,
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
        }
    }

    pub fn new_pop(name: String, l1: i32, l2: i32, h1: i32, h2: i32) -> Field {
        Field{
            name: name,
            lower1: l1,
            lower2: l2,
            higher1: h1,
            higher2: h2,
        }
    }

    pub fn valid(&self, val: i32) -> bool {
        (val >= self.lower1 && val <= self.higher1) || (val >= self.lower2 && val <= self.higher2)
    }

    pub fn parse_rule(rule: String) -> Field {
        println!("Parsing rule: {}", rule);
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
}


fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let mut fields: Vec<Field> = Vec::new();

    //Will always have 3 blocks separated by "\n\n" characters
    let mut blocks = text.split("\n\n");
    let rules = blocks.next().unwrap();
    let (_, _own) = blocks.next().unwrap().split_at(13);    //hardcoded splits to remove labels
    let (_, others) = blocks.next().unwrap().split_at(16);  //hardcoded splits to remove labels

    for rule in rules.lines() {
        fields.push(Field::parse_rule(String::from(rule)));
    }

    let mut error = 0;
    //for each ticket
    for (i, ticket) in others.lines().enumerate() {
        //check every value in the ticket
        for val in ticket.split(",") {
            //check each value against every rule to see if it passes at least one
            let v2 = val.parse::<i32>().unwrap();
            let mut passes = false;
            for field in fields.iter() {
                if field.valid(v2) {
                    passes = true;
                    break;
                }
            }
            if !passes {
                error += v2;
                println!("value {} in ticket {} fails to pass", v2, i+26);
            }
        }
    }

    println!("The error is: {}", error);

    Ok(())
}