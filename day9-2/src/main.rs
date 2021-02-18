use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let mut data: Vec<i64> = Vec::new();

    //populate data vector
    for line in text.lines() {
        data.push(line.parse::<i64>().unwrap_or(0));
    }

    let error = 90433990;
    let mut size = 2;
    let mut min = i64::MAX;
    let mut max = i64::MIN;
    let mut done = false;

    while !done {
        //iterate over all slices of size size
        for i in size..data.len()+1 {
            let slice = &data[i-size..i];

            if slice.iter().sum::<i64>() == error {

                min = *slice.iter().min().unwrap_or(&min);
                max = *slice.iter().max().unwrap_or(&max);

                done = true;
                break;
            }
        }
        size += 1;
    }

    println!("The size is {}, min: {}, max: {}, sum: {}", size-1, min, max, min+max);

    Ok(())
}