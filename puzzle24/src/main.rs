use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let mut x = 0;
    let mut y = 0;
    let mut wayx = 10;
    let mut wayy = 1;

    for line in text.lines() {
        let (op, val) = line.split_at(1);
        let val = val.parse::<i32>().unwrap_or(0);

        println!("x: {}, y: {}, wayx: {}, wayy: {}", x, y, wayx, wayy);

        match op {
            "N" => wayy += val,
            "S" => wayy -= val,
            "E" => wayx += val,
            "W" => wayx -= val,
            "F" => {
                x += wayx*val;
                y += wayy*val;
            }
            "L" => {
                match val {
                    90 => {
                        let tmp = wayx;
                        wayx = -wayy;
                        wayy = tmp;
                    }

                    180 => {
                        wayx = -wayx;
                        wayy = -wayy;
                    }

                    270 => {
                        let tmp = wayx;
                        wayx = wayy;
                        wayy = -tmp;
                    }

                    _ => ()
                }

            }
            "R" => {
                match val {
                    270 => {
                        let tmp = wayx;
                        wayx = -wayy;
                        wayy = tmp;
                    }

                    180 => {
                        wayx = -wayx;
                        wayy = -wayy;
                    }

                    90 => {
                        let tmp = wayx;
                        wayx = wayy;
                        wayy = -tmp;
                    }

                    _ => ()
                }
            }

            &_  => panic!("Unexpected character {}", op),
        }
    }

    println!("x: {}, y: {}, wayx: {}, wayy: {}", x, y, wayx, wayy);

    println!("Manhatten distance: {}", x.abs() + y.abs());

    Ok(())
}