use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    //load the numbers into a data vec
    let mut v: Vec<i32> = Vec::new();
    for line in text.lines() {
        v.push(line.parse::<i32>().unwrap());
    }

    //sort the array
    v.sort_unstable();
    //println!("{:#?}", v);

    //iterate over the array, and count 1 gaps and 3 gaps
    let mut jolt3 = 1; //gap to phone
    let mut jolt1 = 0;
    let mut prev = 0;
    for i in v.iter() {
        if i-prev == 1{
            //println!("{} - {} has gap of 1", i, prev);
            jolt1 += 1;
        } else if i-prev ==3 {
            //println!("{} - {} has gap of 3", i, prev);
            jolt3 += 1;
        } else {
            //println!("{} - {} has gap of 2", i, prev);
        }
        prev = *i;
    }

    println!("jolt1: {}, jolt3: {}, product: {}", jolt1, jolt3, jolt1*jolt3);

    Ok(())
}