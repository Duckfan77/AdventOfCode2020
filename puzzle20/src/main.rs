use std::fs::File;
use std::io::prelude::*;

//slice contains elements between low and high, must bridge gap between low and high
fn _waysbetween(slice: &[i32], _low: i32, _high: i32) -> i128 {

    //slices of size 3 or less are just how many ways can you pick from the slice
    //1 way to pick 0, 3 ways to pick 1, 3 ways to pick 2, 1 way to pick 3
    //for slices of size 3 or less,
    if slice.len() < 3 {
        return 2i128.pow(slice.len() as u32);
    }

    if slice.len() == 3 {
        return 3+3+1;
    }

    //slices of size 4 need to select at least 1, but any 1 will work, as then will be in range of the 3gap which is max allowed
    //4 ways to pick 1, 6 ways to pick 2, 4 ways to pick 3, 1 way to pick 4
    if slice.len() == 4 {
        return 4+6+4+1;
    }

    println!("Unexpected Slice Length, ERROR");
    return 0;
}

//reimplemetation of _waysbetween using just the length of the run of 1s
fn options_from_len(run: u32) -> i128 {
    if run < 3 {
        return 2i128.pow(run);
    }

    if run == 3{
        return 3+3+1;
    }

    if run == 4 {
        return 4+6+4+1;
    }

    println!("Unexpected run length, ERROR");
    return 0;
}

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
    v.push(v[v.len()-1]+3);//add phone to list
    //println!("{:#?}", v);

    //Idea 1: brute force test all 2^90 possible inclusion sets, each one can either be in or out of the configuration, seems excessive and time consuming

    //Idea 2: find number of ways between each gap of 3, since those need to be reached, there's no other way past them, then multiply those counts

    //Iterate over the array, find the length of runs of 1 gaps, as 2-gaps don't exist in the input
    let mut maxj1 = 0;
    let mut runj1 = 0;
    let mut prev = 0;
    let mut running_ways = 1;
    for i in v.iter() {
        if i-prev == 1{
            println!("{} - {} has gap of 1", i, prev);
            runj1 += 1;
        } else if i-prev ==3 {
            println!("{} - {} has gap of 3", i, prev);
            println!("Run length: {}", runj1);
            if maxj1 < runj1 {
                maxj1 = runj1;
            }
            //must subract 1 from runj1, as the first one is required as the step from the previous 3-gap, but can't go negative
            running_ways *= options_from_len(if runj1==0 {0} else {runj1-1});
            runj1 = 0;
        } else {
            //println!("{} - {} has gap of 1", i, prev);
            println!("Run length: {}", runj1);
            if maxj1 < runj1 {
                maxj1 = runj1;
            }
            //must subract 1 from runj1, as the first one is required as the step from the previous 3-gap, but can't go negative
            running_ways *= options_from_len(if runj1==0 {0} else {runj1-1});
            runj1 = 0;
        }
        prev = *i;
    }

    println!("Max run of j1, aka, max run between j3s: {}", maxj1);
    println!("Total number of ways to get to phone: {}", running_ways);

    Ok(())
}