use std::fs::File;
use std::io::prelude::*;

const RULE_CNT: usize = 135;

struct Nonterminal {
    pub index: usize,
    pub nprods: u32,
    pub prods: Vec<Production>,
}

struct Production {
    pub unit: bool,
    pub term: char,
    pub nt1: usize,
    pub nt2: usize,
}

impl Nonterminal {
    pub fn new(idx: usize) -> Nonterminal {
        Nonterminal{
            index: idx,
            nprods: 0,
            prods: Vec::new(),
        }
    }

    pub fn empty() -> Nonterminal {
        Nonterminal{
            index: 0,
            nprods: 0,
            prods: Vec::new()
        }
    }

    pub fn add_prod(&mut self, prod: Production) {
        self.prods.push(prod);
        self.nprods += 1;
    }

    pub fn parse_str(rule: &str) -> (usize, Nonterminal) {
        let (name, prods) = rule.split_at(rule.find(':').unwrap());
        let name = name.parse::<usize>().unwrap();
        let mut nt = Nonterminal::new(name);
        let prods = prods[1..].trim();
        for prod in prods.split('|') {
            nt.add_prod(Production::new_from_str(prod.trim()));
        }

        (name, nt)
    }
}

impl Production {
    pub fn new_unit(term: char) -> Production {
        Production{
            unit: true,
            term: term,
            nt1: 0,
            nt2: 0,
        }
    }

    pub fn new_prod(nt1: usize, nt2: usize) -> Production {
        Production{
            unit:false,
            term: '\0',
            nt1: nt1,
            nt2: nt2,
        }
    }

    pub fn new_from_str(rule: &str) -> Production {
        //println!("Making Prod from rule: {}.", rule);
        if rule.contains('"') { //unit production
            Production::new_unit(rule[1..].chars().next().unwrap())
        } else { //normal production
            let (n1, n2) = rule.split_at(rule.find(' ').unwrap());

            //println!("n1: {}, n2: {}.", n1, n2);

            let n1 = n1.trim().parse::<usize>().unwrap();
            let n2 = n2.trim().parse::<usize>().unwrap();

            Production::new_prod(n1, n2)
        }
    }
}

fn CYK(msg: &str, grammar: &Vec<Nonterminal>) -> bool {
    let n = msg.len();
    let r = RULE_CNT;

    //index order is n, n, r, so build backwards
    let mut P = vec![vec![vec![false; r]; n]; n];

    for (s, c) in msg.chars().enumerate() {
        for nt in grammar.iter() {
            for prod in nt.prods.iter() {
                if prod.unit {
                    if prod.term == c{
                        //println!("Set [0, {}, {}]", s, nt.index);
                        P[0][s][nt.index] = true;
                    }
                }
            }
        }
    }

    for l in 2..n+1 { //length of span
        for s in 1..n-l+2 { //start of span
            for p in 1..l { //partition of span
                for nt in grammar.iter() {
                    for prod in nt.prods.iter() {
                        if !prod.unit {
                            //println!("Testing p, s, l: {}, {}, {}", p, s, l);
                            if P[p-1][s-1][prod.nt1] && P[l-p-1][s+p-1][prod.nt2] {
                                //println!("Set [{}, {}, {}]", l-1, s-1, nt.index);
                                P[l-1][s-1][nt.index] = true
                            }
                        }
                    }
                }
            }
        }
    }

    return P[n-1][0][0]
}

fn main() -> std::io::Result<()>{
    let mut file : File = File::open("input")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    //create vector of nonterminals, known size
    let mut grammar: Vec<Nonterminal> = Vec::with_capacity(RULE_CNT);
    for _ in 0..RULE_CNT {
        grammar.push(Nonterminal::empty());
    }

    let msgs = String::from(text.split_off(text.find("\n\n").unwrap()).trim());

    //parse rules
    for rule in text.lines() {
        let (idx, prod) = Nonterminal::parse_str(rule);
        grammar[idx] = prod;
    }

    //rule 8 and 86 don't exist, should be skipped - happened in making the input fully CNF
    let mut total = 0;
    for msg in msgs.lines() {
        println!("Testing line {}", msg);
        total += if CYK(msg, &grammar) {1} else {0};
    }

    println!("Total that pass: {}", total);

    Ok(())
}