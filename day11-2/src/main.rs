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

    let mut outarray: [[Space; WIDTH]; HEIGHT] = [[Space::Empty; WIDTH]; HEIGHT];
    let mut changed = false;

    for i in 1..(HEIGHT-1) as i32 {
        for j in 1..(WIDTH-1) as i32 {

            //Iterate along each axis from the cell
            let mut countfull = 0;
            for (voff, hoff) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)].iter() {
                let mut y = i;
                let mut x = j;
                let mut chair = false;

                //iterate along the axis defined by (voff, hoff), to find first chair
                while !chair {
                    y += voff;
                    x += hoff;

                    match array[y as usize][x as usize] {
                        Space::Occupied => {
                            chair = true;
                            countfull += 1;
                        },
                        Space::Floor => (),
                        Space::Empty => {
                            chair = true;
                        }
                    }
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
                    if countfull >= 5 {
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

    //95 elements in a row, 98 rows, add padding of Empty to make math easier, don't need to deal with edge detection
    let mut array: [[Space; WIDTH]; HEIGHT] = [[Space::Empty; WIDTH]; HEIGHT];

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