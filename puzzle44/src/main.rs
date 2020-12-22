use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;


/**
 * Plays a game of recursive combat
 * Returns true if p1 wins, false if p2 wins
 * The tuple consists of p1wins as bool, then the deck of p1 at the end and deck of p2 at the end
 */
fn playgame(mut d1: VecDeque<i32>, mut d2: VecDeque<i32>, g: i32) -> (bool, VecDeque<i32>, VecDeque<i32>) {
    println!("=== Game {} ===", g);
    let mut r = 0;
    let mut history: Vec<(Vec<i32>, Vec<i32>)> = Vec::new();
    while d2.len() != 0 && d1.len() != 0 {
        r += 1;
        //println!("\n-- Round {} (Game {}) --", r, g);
        //println!("P1's deck: {:?}", d1);
        //println!("P2's deck: {:?}", d2);

        let mut h1: Vec<i32> = Vec::new();
        for c in d1.iter() {
            h1.push(*c);
        }
        let mut h2: Vec<i32> = Vec::new();
        for c in d2.iter() {
            h2.push(*c);
        }

        if history.contains(&(h1.clone(), h2.clone())) {
            println!("Repeat, player 1 wins game {}", g);
            return (true, d1, d2)
        }

        history.push((h1, h2));

        let c1 = d1.pop_front().unwrap();
        let c2 = d2.pop_front().unwrap();

        //println!("P1 plays: {}\nP2 plays: {}", c1, c2);

        //check if recurse
        if c1 > d1.len() as i32 || c2 > d2.len() as i32{//can't recurse, not enough cards
            if c1 > c2 {
                //println!("P1 wins round {} of game {}!", r, g);
                d1.push_back(c1);
                d1.push_back(c2);
            } else {
                //println!("P2 wins round {} of game {}!", r, g);
                d2.push_back(c2);
                d2.push_back(c1);
            }
        } else { //recurse
            //create VecDeques for next level down
            //println!("Playing a sub-game to determine the winner...\n");
            let mut d1p: VecDeque<i32> = VecDeque::new();
            for (i, c) in d1.iter().enumerate() {
                if i == c1 as usize{
                    break;
                }
                d1p.push_back(*c);
            }

            let mut d2p: VecDeque<i32> = VecDeque::new();
            for (i, c) in d2.iter().enumerate() {
                if i == c2 as usize {
                    break;
                }
                d2p.push_back(*c);
            }

            //Find winner using recursive game
            let (p1win, _, _) = playgame(d1p, d2p, g+1);
            if p1win {
                //println!("\n...anyway, back to game 1.\n Player 1 wins round {} of game {}!", r, g);
                d1.push_back(c1);
                d1.push_back(c2);
            } else {
                //println!("\n...anyway, back to game 1.\n Player 2 wins round {} of game {}!", r, g);
                d2.push_back(c2);
                d2.push_back(c1);
            }
        }
    }

    if d2.len() == 0 { //p1 win
        println!("The winner of game {} is player 1", g);
        (true, d1, d2)
    } else {
        println!("The winner of game {} is player 2", g);
        (false, d1, d2)
    }
}

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

    let (p1won, p1end, p2end) = playgame(p1, p2, 1);

    //find who won
    let win = if !p1won {&p2end} else {&p1end};

    let mut total = 0;
    for (index, card) in win.iter().rev().enumerate() {
        total += (index as i32 + 1)*card;
    }

    println!("Total score at the end: {}", total);

    Ok(())
}