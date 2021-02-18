use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;

fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let mut p1: VecDeque<i32> = VecDeque::new();
    let mut p2: VecDeque<i32> = VecDeque::new();

    let d2 = text.split_off(text.find("\n\n").unwrap());
    let d2 = &d2.trim()[10..];//[10..] is to trim off the "Player 1:" line
    let d1 = &text.trim()[10..];

    //populate the two queues
    for line in d1.lines() {
        p1.push_back(line.parse::<i32>().unwrap());
    }
    for line in d2.lines() {
        p2.push_back(line.parse::<i32>().unwrap());
    }

    //execute rules until one deck is empty
    while p2.len() != 0 && p1.len() != 0 {
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        }else{
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }

    //find who won
    let win = if p2.len() != 0 {&p2} else {&p1};

    let mut total = 0;
    for (index, card) in win.iter().rev().enumerate() {
        total += (index as i32 + 1)*card;
    }

    println!("Total score at the end: {}", total);

    Ok(())
}