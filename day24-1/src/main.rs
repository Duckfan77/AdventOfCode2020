use std::fs::File;
use std::io::prelude::*;
use num_complex::Complex;
use std::collections::HashSet;

fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    //replace two letter symbols to single letter symbols for easier parsing
    text = text.replace("nw", "Q").replace("sw", "Z").replace("ne", "E").replace("se", "C");

    let mut sparse_grid: HashSet<Complex<i32>> = HashSet::new();
    for line in text.lines() {
        //r axis is real, q axis is imaginary
        //positive r is east, positive q is se
        let mut tile = Complex::new(0i32, 0);
        for c in line.chars() {
            tile += match c {
                'e' => Complex::new(1i32, 0),
                'w' => Complex::new(-1i32, 0),
                'C' => Complex::new(0i32, 1),
                'Q' => Complex::new(0i32, -1),
                'E' => Complex::new(1i32, -1),
                'Z' => Complex::new(-1i32, 1),
                 _  => panic!("Unexpected character: {}", c)
            }
        }

        if sparse_grid.contains(&tile) {
            sparse_grid.remove(&tile);
        } else {
            sparse_grid.insert(tile);
        }
    }

    println!("Total number of black tiles: {}", sparse_grid.len());

    Ok(())
}