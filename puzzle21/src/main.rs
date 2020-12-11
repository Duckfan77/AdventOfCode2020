use std::fs::File;
use std::io::prelude::*;

enum Space {
    Floor,
    Occupied,
    Empty,
}

fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    //get height and length of the array we will create
    let mut iter = text.lines();
    let height = iter.clone().count();
    let len = iter.next().unwrap().len();

    let array: [[i32; 1]; 2] = [[0],[1]];

    println!("{:#?}", array);

    Ok(())
}