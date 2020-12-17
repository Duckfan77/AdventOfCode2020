use std::fs::File;
use std::io::prelude::*;

const SIZE1: usize = 30;
const SIZE2: usize = 30;
const CENTER1: usize = SIZE1/2;
const CENTER2: usize = SIZE2/2;

fn iterate_array(array: Vec<Vec<Vec<Vec<bool>>>>) -> (Vec<Vec<Vec<Vec<bool>>>>, bool) {

    let mut outarray = vec![vec![vec![vec![false; SIZE2]; SIZE2]; SIZE1]; SIZE1];
    let mut changed = false;

    for i in 1..(SIZE2-1) as i32 {
        for j in 1..(SIZE2-1) as i32 {
            for k in 1..(SIZE1-1) as i32 {
                for l in 1..(SIZE1-1) as i32 {
                    //get number full around the cell
                    let mut countfull = 0;
                    for xoff in -1..2 {
                        for yoff in -1..2 {
                            for zoff in -1..2 {
                                for woff in -1..2 {
                                    //don't look at 0,0,0,0
                                    if xoff==0 && yoff==0 && zoff==0 && woff==0{
                                        continue;
                                    }
                                    //println!("i: {}, j: {}, k: {}, l: {}", i, j, k, l);
                                    countfull += if array[(i+xoff) as usize][(j+yoff) as usize][(k+zoff) as usize][(l+woff) as usize] {1} else {0};
                                }
                            }
                        }
                    }

                    //implement transition rules, track if a state change has happenes
                    outarray[i as usize][j as usize][k as usize][l as usize] = match array[i as usize][j as usize][k as usize][l as usize] {
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
    }

    return (outarray, changed)
}


fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    //initialize array to empty
    let mut space = vec![vec![vec![vec![false; SIZE2]; SIZE2]; SIZE1]; SIZE1];
    //let mut space: Box<[[[[bool; SIZE2]; SIZE2]; SIZE1]; SIZE1]> = Box::new([[[[false; SIZE2]; SIZE2]; SIZE1]; SIZE1]);

    //populate space from file
    for (i, line) in text.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            space[CENTER2+i][CENTER2+j][CENTER1][CENTER1] = match c {
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
    for cube in space.iter(){
        for slice in cube.iter() {
            for row in slice.iter() {
                for cell in row.iter() {
                    count += if *cell {1} else {0}
                }
            }
        }
    }

    println!("In total, {} cells are active at the end", count);

    Ok(())
}