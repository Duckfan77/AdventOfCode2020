use std::fs::File;
use std::io::prelude::*;
#[macro_use] extern crate lazy_static;

use regex::Regex;

/**
 * Checks validity of byr field contents
 *
 * Pass in the contents of the byr field, after the ':'
 * Returns 1 if valid, 0 if invald
 */
fn check_byr(field : &str) -> u8 {
    if field.len() != 4 {return 0;}
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^((19[2-9][0-9])|(200[0-2]))$").unwrap();
    }
    if RE.is_match(field) {
        return 1
    } else {
        //println!("Didn't Match {} for byr", field);
        return 0
    }
    //regex for year: ^((19[2-9][0-9])|(200[0-2]))$
}

/**
 * Checks validity of iyr field contents
 *
 * Pass in the contents of the iyr field, after the ':'
 * Returns 1 if valid, 0 if invald
 */
fn check_iyr(field : &str) -> u8 {
    if field.len() != 4 {return 0;}
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^((201[0-9])|2020)$").unwrap();
    }
    if RE.is_match(field) {
        return 1
    } else {
        //println!("Didn't Match {} for iyr", field);
        return 0
    }
    //regex for year: ^((201[0-9])|2020)$
}

/**
 * Checks validity of eyr field contents
 *
 * Pass in the contents of the eyr field, after the ':'
 * Returns 1 if valid, 0 if invald
 */
fn check_eyr(field : &str) -> u8 {
    if field.len() != 4 {return 0;}
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^((202[0-9])|2030)$").unwrap();
    }
    if RE.is_match(field) {
        return 1
    } else {
        //println!("Didn't Match {} for eyr", field);
        return 0
    }
    //regex for year: ^((202[0-9])|2030)$
}

/**
 * Checks validity of hgt field contents
 *
 * Pass in the contents of the hgt field, after the ':'
 * Returns 1 if valid, 0 if invald
 */
fn check_hgt(field : &str) -> u8 {
    if field.len() == 4 {
        //in
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(59|6[0-9]|7[0-6])in$").unwrap();
        }
        if RE.is_match(field) {
            return 1
        } else {
            //println!("Didn't Match {} for hgt (in)", field);
            return 0
        }
        //regex for in: ^(59|6[0-9]|7[0-6])in$
    } else if field.len()==5 {
        //cm
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^1([5-8][0-9]|9[0-3])cm$").unwrap();
        }
        if RE.is_match(field) {
            return 1
        } else {
            //println!("Didn't Match {} for hgt (cm)", field);
            return 0
        }
        //regex for cm: ^1([5-8][0-9]|9[0-3])cm$
    } else {// bad length
        //println!("Didn't Match {} for hgt (bad len)", field);
        return 0
    }
}

/**
 * Checks validity of hcl field contents
 *
 * Pass in the contents of the hcl field, after the ':'
 * Returns 1 if valid, 0 if invald
 */
fn check_hcl(field : &str) -> u8 {
    if field.len() != 7 {return 0;}
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }
    if RE.is_match(field) {
        return 1
    } else {
        //println!("Didn't Match {} for hcl", field);
        return 0
    }
    //regex for hair color: ^#[0-9a-f]{6}$
}

/**
 * Checks validity of ecl field contents
 *
 * Pass in the contents of the ecl field, after the ':'
 * Returns 1 if valid, 0 if invald
 */
fn check_ecl(field : &str) -> u8 {
    match field {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => 1,
        &_ => {
            //println!("Didn't Match {} for ecl", field);
            0
        },
    }
}

/**
 * Checks validity of pid field contents
 *
 * Pass in the contents of the pid field, after the ':'
 * Returns 1 if valid, 0 if invald
 */
fn check_pid(field : &str) -> u8 {
    if field.len() != 9 {return 0;}
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }
    if RE.is_match(field) {
        return 1
    } else {
        //println!("Didn't Match {} for pid", field);
        return 0
    }
    //regex for pid: ^[0-9]{9}$
}

fn check_passport(block : &str) -> bool {
    //byr, iyr, eyr, hgt, hcl, ecl, pid, unknownField
    let mut fields: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

    //split the block at spaces and newlines to get the fields
    for field in block.split_terminator(|c| c==' ' || c=='\n') {
        //println!("{}", field);
        match &field[0..3] {
            "byr" => fields[0] += check_byr(&field[4..]),
            "iyr" => fields[1] += check_iyr(&field[4..]),
            "eyr" => fields[2] += check_eyr(&field[4..]),
            "hgt" => fields[3] += check_hgt(&field[4..]),
            "hcl" => fields[4] += check_hcl(&field[4..]),
            "ecl" => fields[5] += check_ecl(&field[4..]),
            "pid" => fields[6] += check_pid(&field[4..]),
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
