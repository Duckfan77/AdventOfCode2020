fn main() -> std::io::Result<()>{
    let mut cups = vec![2,8,4,5,7,3,9,6,1];
    for v in 10..1000001 {
        cups.push(v);
    }
    println!("Number of cups total: {}, {:?}", cups.len(), cups[cups.len()-1]);

    let max = *cups.iter().max().unwrap();
    let min = *cups.iter().min().unwrap();

    //construct a vec to serve as a "linked list" telling you where each cup points one clockwise
    let mut list = vec![0; max+1];
    (0..cups.len()).for_each(|i| list[cups[i]] = cups[(i+1) % cups.len()]);

    let mut cur=cups[0];
    for i in 1..10000001 {
        if i%10000 == 0 {
            println!("turn {}", i);
        }

        //println!("-- move {} --\ncups: {:?}\ncurrent index: {}, current label: {}", i, cups, cur, cur_label);

        //remove 3
        //get the three to be removed
        let p1 = list[cur];
        let p2 = list[p1];
        let p3 = list[p2];

        //skip the three removed in the circle
        list[cur] = list[p3];

        //println!("Pick up: {:?}", removed);

        //select destination
        let mut dest = cur-1;
        if dest < min {
            dest = max;
        }
        while [p1, p2, p3].contains(&dest) {
            dest -= 1;
            if dest < min {
                dest = max;
            }
        }

        //println!("destination: {}\n", dest);

        //Insert cups
        list[p3] = list[dest];
        list[dest] = p1;

        //update current
        cur = list[cur];
    }

    //println!{"-- final --\ncups: {:?}, current index: {}", cups, cur};
    let r = list[1];
    let rr = list[r];
    println!("r: {}, rr: {}, product: {}", r, rr, r*rr);

    Ok(())
}