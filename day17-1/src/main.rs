use std::fs::File;
use std::io::prelude::*;

const SIZE: usize = 50;
const CENTER: usize = SIZE/2;

fn iterate_array(array: [[[bool; SIZE]; SIZE]; SIZE]) -> ([[[bool; SIZE]; SIZE]; SIZE], bool) {

    let mut outarray: [[[bool; SIZE]; SIZE]; SIZE] = [[[false; SIZE]; SIZE]; SIZE];
    let mut changed = false;

    for i in 1..(SIZE-1) as i32 {
        for j in 1..(SIZE-1) as i32 {
            for k in 1..(SIZE-1) as i32 {
                //get number full around the cell
                let mut countfull = 0;
                for xoff in -1..2 {
                    for yoff in -1..2 {
                        for zoff in -1..2 {
                            //don't look at 0,0,0
                            if xoff==0 && yoff==0 && zoff==0 {
                                continue;
                            }

                            countfull += if array[(i+xoff) as usize][(j+yoff) as usize][(k+zoff) as usize] {1} else {0};
                        }
                    }
                }

                //implement transition rules, track if a state change has happenes
                outarray[i as usize][j as usize][k as usize] = match array[i as usize][j as usize][k as usize] {
                    true => {
                        if countfull == 2 || countfull == 3 {
                            true
                        } else {
                            changed = true;
                            false
                        }
                    },

                    false => {
                        if countfull == 3 {
                            changed = true;
                            true
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }

    return (outarray, changed)
}


fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    //initialize array to empty
    let mut space: [[[bool; SIZE]; SIZE]; SIZE] = [[[false; SIZE]; SIZE]; SIZE];

    //populate space from file
    for (i, line) in text.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            space[CENTER+i][CENTER+j][CENTER] = match c {
                '.' => false,
                '#' => true,
                 _  => panic!("Unexpected character in test: {}", c),
            }
        }
    }

    //iterate 6 times
    for i in 0..6 {
        let (tempspace, _) = iterate_array(space);
        space = tempspace;
        println!("Complete Cycle {}", i);
    }

    //count active cells at the end
    let mut count = 0;
    for slice in space.iter() {
        for row in slice.iter() {
            for cell in row.iter() {
                count += if *cell {1} else {0}
            }
        }
    }

    println!("In total, {} cells are active at the end", count);

    Ok(())
}