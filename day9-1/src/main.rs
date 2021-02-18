use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let mut data: Vec<i32> = Vec::new();

    //populate data vector
    for line in text.lines() {
        data.push(line.parse::<i32>().unwrap_or(0));
    }

    let mut el: i32=0;

    let pre = 25;

    for (i, el1) in data[pre..].iter().enumerate() {
        let mut good = false;

        el = *el1;

        let slice = &data[i..i+pre];

        for (j, el2) in slice.iter().enumerate() {
            for el3 in slice[j+1..].iter() {
                if *el1 == el2+el3 {
                    good = true;
                }
            }
        }

        if !good {
            println!("No match found, exiting");
            break;
        }
    }

    println!("{} is the element with no match", el);

    Ok(())
}