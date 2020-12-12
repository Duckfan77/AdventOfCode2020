use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum Dir {
    East,
    North,
    South,
    West
}

fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let mut dir = Dir::East;
    let mut x = 0;
    let mut y = 0;

    for line in text.lines() {
        let (op, val) = line.split_at(1);
        let val = val.parse::<i32>().unwrap_or(0);

        println!("x: {}, y: {}, dir: {:?}", x, y, dir);

        match op {
            "N" => y += val,
            "S" => y -= val,
            "E" => x += val,
            "W" => x -= val,
            "F" => {
                match dir {
                    Dir::East => x += val,
                    Dir::North => y += val,
                    Dir::South => y -= val,
                    Dir::West => x -= val,
                }
            }
            "R" => {
                dir = match (dir, val) {
                    (Dir::East, 90) => Dir::South,
                    (Dir::East, 180) => Dir::West,
                    (Dir::East, 270) => Dir::North,
                    (Dir::East, _)  => Dir::East,

                    (Dir::North, 90) => Dir::East,
                    (Dir::North, 180) => Dir::South,
                    (Dir::North, 270) => Dir::West,
                    (Dir::North, _)  => Dir::North,

                    (Dir::South, 90) => Dir::West,
                    (Dir::South, 180) => Dir::North,
                    (Dir::South, 270) => Dir::East,
                    (Dir::South, _)  => Dir::South,

                    (Dir::West, 90) => Dir::North,
                    (Dir::West, 180) => Dir::East,
                    (Dir::West, 270) => Dir::South,
                    (Dir::West, _)  => Dir::West,
                }
            }
            "L" => {
                dir = match (dir, val) {
                    (Dir::East, 90) => Dir::North,
                    (Dir::East, 180) => Dir::West,
                    (Dir::East, 270) => Dir::South,
                    (Dir::East, _)  => Dir::East,

                    (Dir::North, 90) => Dir::West,
                    (Dir::North, 180) => Dir::South,
                    (Dir::North, 270) => Dir::East,
                    (Dir::North, _)  => Dir::North,

                    (Dir::South, 90) => Dir::East,
                    (Dir::South, 180) => Dir::North,
                    (Dir::South, 270) => Dir::West,
                    (Dir::South, _)  => Dir::South,

                    (Dir::West, 90) => Dir::South,
                    (Dir::West, 180) => Dir::East,
                    (Dir::West, 270) => Dir::North,
                    (Dir::West, _)  => Dir::West,
                }
            }

            &_  => panic!("Unexpected character {}", op),
        }
    }

    println!("x: {}, y: {}, dir: {:?}", x, y, dir);

    println!("Manhatten distance: {}", x.abs() + y.abs());

    Ok(())
}