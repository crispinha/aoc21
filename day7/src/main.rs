use std::fs::File;
use std::io::Read;
use std::str::FromStr;

type Num = i64;

fn cost_of(crabs: &Vec<Num>, target: Num) -> Num {
    let mut cost = 0;
    for c in crabs {
        cost += Num::abs(c - target)
    }

    cost
}

fn cost_of_2(crabs: &Vec<Num>, target: Num) -> Num {
    let mut cost = 0;
    for c in crabs {
        let distance = Num::abs(c - target);
        let thing = (distance * (distance + 1)) / 2;
        cost += thing;
    }

    cost
}


fn calc1(crabs: Vec<Num>) -> Option<()> {
    let mut min = Num::MAX;
    let max_pos = crabs.iter().max().or(Option::None)?;
    for p in 0..=*max_pos {
        let cost = cost_of(&crabs, p);
        if cost < min {
            min = cost;
        }
    }
    println!("min: {}", min);
    Some(())
}

fn calc2(crabs: Vec<Num>) -> Option<()> {
    let mut min = Num::MAX;
    let max_pos = crabs.iter().max().or(Option::None)?;
    for p in 0..=*max_pos {
        let cost = cost_of_2(&crabs, p);
        if cost < min {
            min = cost;
        }
    }
    println!("min: {}", min);
    Some(())
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("ch.txt")?;
    let mut stuff = String::new();

    file.read_to_string(&mut stuff)?;

    let raw_crabs: Vec<Num> = stuff.split(",").map(|x|{Num::from_str(x).unwrap()}).collect();
    // let n = raw_crabs.iter().max().ok_or(0);
    // println!("{:#?}", n);
    calc2(raw_crabs);

    Ok(())
}
