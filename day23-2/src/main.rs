fn main() -> std::io::Result<()>{
    let mut cups = vec![2,8,4,5,7,3,9,6,1];
    let mut removed: Vec<i32> = Vec::new();

    let mut cur=0;
    for i in 1..101 {
        let cur_label = cups[cur];

        println!("-- move {} --\ncups: {:?}\ncurrent index: {}, current label: {}", i, cups, cur, cur_label);

        //remove 3
        for _ in 0..3 {
            if cur+1 < cups.len() {
                removed.push(cups.remove(cur+1));
            } else {
                removed.push(cups.remove(0));
            }
        }

        println!("Pick up: {:?}", removed);

        //select destination
        let mut dest = cur_label-1;
        if dest < 1 {
            dest = 9;
        }
        while removed.contains(&dest) {
            dest -= 1;
            if dest < 1 {
                dest = 9;
            }
        }

        println!("destination: {}\n", dest);

        //Insert cups
        let dest_addr = cups.iter().position(|x| *x == dest).unwrap()+1;
        for c in removed.drain(..).rev() {
            cups.insert(dest_addr, c);
        }

        //update current
        cur = cups.iter().position(|x| *x == cur_label).unwrap()+1;
        if cur >= cups.len() {
            cur = 0;
        }
    }

    println!{"-- final --\ncups: {:?}, current index: {}", cups, cur};

    Ok(())
}