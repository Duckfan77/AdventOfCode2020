use std::fs::File;
use std::io::prelude::*;

struct Tile {
    pub body: String,
    pub edge1: i32,
    pub edge1r: i32,
    pub edge2: i32,
    pub edge2r: i32,
    pub edge3: i32,
    pub edge3r: i32,
    pub edge4: i32,
    pub edge4r: i32,
    pub id: i32,
}

impl Tile {
    /**
     * Convert edge string to int, both forward and reverse.
     */
    fn get_edge(e: &str) -> (i32, i32) {
        let mut ed = 0;
        let mut ed2 = 0;
        for c in e.chars() {
            ed <<= 1;
            ed |= match c {
                '.' => {0},
                '#' => {1},
                 _  => {panic!("Error, found character {}", c)},
            }
        }

        for c in e.chars().rev() {
            ed2 <<= 1;
            ed2 |= match c {
                '.' => {0},
                '#' => {1},
                 _  => {panic!("Error, found character {}", c)},
            }
        }

        (ed, ed2)
    }

    pub fn new(body: &str) -> Tile {
        let id = body[5..9].parse::<i32>().unwrap();

        ()

        Tile{
            body: String::from(body),
            edge1: 0,
            edge2: 0,
            edge3: 0,
            edge4: 0,
            id: id,
        }
    }
}

fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    for t in text.split_terminator("\n\n") {
        Tile::new(t);
    }

    println!("{}", Tile::get_edge(".##.#..#.."));

    Ok(())
}