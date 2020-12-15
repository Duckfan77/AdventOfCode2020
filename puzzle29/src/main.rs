use std::collections::HashMap;

fn main() -> std::io::Result<()>{
    let start: [i32; 6] = [14, 8, 16, 0, 1, 17];

    let mut turn = 0;
    let mut last = 0;
    let mut store: HashMap<i32, i32> = HashMap::new();
    let mut first = true;

    //go over start list
    for i in start.iter() {
        if !first {
            store.insert(last, turn-1);
        } else {
            first = false;
        }
        last = *i;
        turn += 1;
    }

    while turn < 2020 {
        //println!("turn: {}, last: {}", turn, last);
        //last hasn't been inserted yet, can check presence easily
        if store.contains_key(&last) { //has been seen before
            let past = store.remove(&last).unwrap();
            store.insert(last, turn-1);
            //calculate distance between and use as next number
            last = turn-past-1;
        } else { //hasn't been seen before
            store.insert(last, turn-1);
            last = 0;
        }
        turn += 1;
    }

    println!("2020th: {}", last);

    Ok(())
}