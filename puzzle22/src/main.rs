use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Space {
    Floor,
    Occupied,
    Empty,
}

const HEIGHT: usize = 100;
const WIDTH: usize = 97;

fn iterate_array(array: [[Space; WIDTH]; HEIGHT]) -> ([[Space; WIDTH]; HEIGHT], bool) {

    let mut outarray: [[Space; WIDTH]; HEIGHT] = [[Space::Floor; WIDTH]; HEIGHT];
    let mut changed = false;

    for i in 1..(HEIGHT-1) as i32 {
        for j in 1..(WIDTH-1) as i32 {
            //get number full around the cell
            let mut countfull = 0;
            for voff in -1..2 {
                for hoff in -1..2 {
                    //don't look at 0,0
                    if voff==0 && hoff==0 {
                        continue;
                    }

                    countfull += if array[(i+voff) as usize][(j+hoff) as usize] == Space::Occupied {1} else {0};
                }
            }

            //implement transition rules, track if a state change has happenes
            outarray[i as usize][j as usize] = match array[i as usize][j as usize] {
                Space::Floor => Space::Floor,

                Space::Empty => {
                    if countfull==0 {
                        changed = true;
                        Space::Occupied
                    }else {
                        Space::Empty
                    }
                }

                Space::Occupied => {
                    if countfull >= 4 {
                        changed = true;
                        Space::Empty
                    } else {
                        Space::Occupied
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

    //95 elements in a row, 98 rows, add padding of floor to make math easier, don't need to deal with edge detection
    let mut array: [[Space; WIDTH]; HEIGHT] = [[Space::Floor; WIDTH]; HEIGHT];

    //populate array
    for (i, line) in text.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            array[i+1][j+1] = match c {
                'L' => Space::Empty,
                '.' => Space::Floor,
                '#' => Space::Occupied,
                 _  => panic!("Unexpected character {}", c),
            };
        }
    }

    let mut changed = true;
    let mut iter = 0;
    while changed {
        println!("Iteration: {}", iter);
        let (temparray, tempchanged) = iterate_array(array);
        changed = tempchanged;
        array = temparray;
        iter +=1;
    }

    let mut count = 0;
    for i in array.iter() {
        for j in i {
            count += if *j == Space::Occupied {1} else {0};
        }
    }

    println!("{} occupied seats at the end", count);

    Ok(())
}