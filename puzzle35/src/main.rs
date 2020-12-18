use std::fs::File;
use std::io::prelude::*;
extern crate mexprp;
use mexprp::*;

#[derive(Debug, Clone, PartialEq)]
struct JankNum {
    val: i64,
}

//Janky implementation of a number that uses sub to implement multiplication
impl JankNum {
    pub fn new(v: i64) -> JankNum {
        JankNum{val: v}
    }
}

impl std::fmt::Display for JankNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.val.fmt(f)
    }
}

impl mexprp::num::Num for JankNum {
    fn from_f64(t: f64, _ctx: &Context<Self>) -> Calculation<Self> {
        Ok(Answer::Single(JankNum::new(t as i64)))
    }

    fn from_f64_complex((r, _i): (f64, f64), _ctx: &Context<Self>) -> Calculation<Self> {
        Ok(Answer::Single(JankNum::new(r as i64)))
    }

    fn typename() -> String {
        String::from("JankNum")
    }

    fn add(&self, other: &Self, _ctx: &Context<Self>) -> Calculation<Self> {
        Ok(Answer::Single(JankNum::new(self.val + other.val)))
    }

    fn sub(&self, other: &Self, _ctx: &Context<Self>) -> Calculation<Self> {
        Ok(Answer::Single(JankNum::new(self.val * other.val)))
    }
}

fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    //replace multiplication with subtraction to cheat order of operations, using a parser with a type that uses subtraction to perform multiplication
    text = text.replace("*", "-");

    let mut total = 0;
    for line in text.lines() {
        let r = eval::<JankNum>(line).unwrap().unwrap_single().val;
        println!("{} Evaluates to {}", line, r);
        total += r;
    }

    println!("Total: {}", total);

    Ok(())
}