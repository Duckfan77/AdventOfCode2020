const SUBJECT_NUMBER: u64 = 7;

fn main() {
    let server_public = 14082811;
    let client_public = 5249543;

    //find client loop size
    let mut ls = 0;
    let mut val: u64 = 1;
    loop {
        val = (val*SUBJECT_NUMBER)%20201227;
        ls += 1;
        if val == client_public {
            break;
        }
    }

    println!("Client loop number: {}", ls);

    val = 1;
    for _i in 0..ls {
        //println!("Cycle: {}", _i);
        val = (val*server_public)%20201227;
    }

    println!("Total value: {}", val);
}
