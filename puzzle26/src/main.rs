use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let (_, rts) = text.split_at(text.find("\n").unwrap());

    let rts = rts.trim();

    //modulus, offset
    let mut v: Vec<(u128, u128)> = Vec::new();
    let mut M: u128 = 1;

    for (i, time) in rts.split(",").enumerate() {
        if time == "x" {
            continue;
        }

        let time = time.parse::<u128>().unwrap();
        println!("i: {}, time: {}, imodt: {}", i,time, ((i as u128)%time));
        v.push((time, time - ((i as u128)%time)));

        M *= time;
    }

    println!("M: {}", M);

    //(37, 0), (41, 27), (601, 37)
    //(19, 11), (17, 3), (23, 14)
    //(29, 8), (443, 68), (13, 3)

    let mut identities: Vec<u128> = Vec::new();

    for (i,(mi, _)) in v.iter().enumerate() {
        let yi = (M/mi) % mi;
        let mut zi: u128 = 0;

        for zip in 0..*mi {
            if zip*yi % mi == 1{
                zi = zip;
                break;
            }
        }

        println!("i: {}, mi: {}, yi: {}, zi: {}", i, mi, yi, zi);
        identities.push(M/mi*zi);
    }

    println!("{:#?}", identities);

    let mut k = 0u128;

    for (i, (_, off)) in v.iter().enumerate() {
        k += off * identities[i];
    }

    println!("t: {}", k % M);

    Ok(())
}