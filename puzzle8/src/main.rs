use std::fs::File;
use std::io::prelude::*;

fn check_passport(block : &str) -> bool {
    //byr, iyr, eyr, hgt, hcl, ecl, pid, unknownField
    let mut fields: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

    //split the block at spaces and newlines to get the fields
    for field in block.split_terminator(|c| c==' ' || c=='\n') {
        //println!("{}", field);
        match &field[0..3] {
            "byr" => fields[0] += 1,
            "iyr" => fields[1] += 1,
            "eyr" => fields[2] += 1,
            "hgt" => fields[3] += 1,
            "hcl" => fields[4] += 1,
            "ecl" => fields[5] += 1,
            "pid" => fields[6] += 1,
            "cid" => (),
            &_ => fields[7] += 1,
        }
    }

    return fields[0]==1 && fields[1]==1 && fields[2]==1 && fields[3]==1 && fields[4]==1 && fields[5]==1 && fields[6]==1 && fields[7]==0;
}

fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let mut n = 0;

    //split at blank lines to get blocks
    for block in text.split_terminator("\n\n") {
        if check_passport(block) {
            n+=1;
        }
    }

    println!("{} accepting passports", n);
    Ok(())
}
