#![allow(dead_code)]
#![allow(unused_variables)]

use crate::Bracket::{Closed, Open};
use crate::BracketType::{Curly, Pointy, Round, Square};
use std::fs::File;
use std::io::Read;
use crate::Out::{Error, Incomplete};

type Num = i64;

#[derive(PartialEq, Debug)]
enum BracketType {
    Round,
    Square,
    Curly,
    Pointy,
}

#[derive(PartialEq, Debug)]
enum Bracket {
    Open(BracketType),
    Closed(BracketType),
}

#[derive(PartialEq, Debug)]
enum Out {
    Error(BracketType),
    Incomplete(Vec<BracketType>)
}

fn is_error(o: &Out) -> bool {
    match o {
        Error(_) => true,
        _ => false
    }
}

fn to_bracket(c: &str) -> Bracket {
    match c {
        "(" => Open(Round),
        "[" => Open(Square),
        "{" => Open(Curly),
        "<" => Open(Pointy),
        ")" => Closed(Round),
        "]" => Closed(Square),
        "}" => Closed(Curly),
        ">" => Closed(Pointy),
        _ => panic!("oh god oh fuck"),
    }
}

fn points(o: Out) -> Num {
    match o {
        Error(bracket) => match bracket {
            Round => 3,
            Square => 57,
            Curly => 1197,
            Pointy => 25137,
        },
        Incomplete(_) => 0
    }

}

fn verify(sentence: Vec<Bracket>) -> Out {
    let mut stack: Vec<BracketType> = vec![];
    for bracket in sentence {
        match bracket {
            Open(b) => stack.push(b),
            Closed(b) => {
                let other = stack.pop().unwrap();
                if b != other {
                    return Error(b);
                }
            }
        }
    }
    Incomplete(stack)
}

fn calc1(data: Vec<Vec<Bracket>>) {
    let result = data
        .into_iter()
        .map(|x| verify(x))
        .map(|x| points(x))
        .reduce(|acc, x| acc + x).unwrap_or(0);
    println!("{}", result);
}

fn points2(b: BracketType) -> Num {
    match b {
        Round => 1,
        Square => 2,
        Curly => 3,
        Pointy => 4
    }
}

fn calc2(data: Vec<Vec<Bracket>>) {
    let mut points: Vec<Num> = vec![];
    for sentence in data {
        let s = match verify(sentence) {
            Error(_) => continue,
            Incomplete(x) => x
        };
        let point = s.into_iter().rev().fold(0, |acc, x| (acc * 5) + points2(x));
        points.push(point);
    }
    points.sort();
    let score = points[points.len()/2];
    println!("{}", score);
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("ch.txt")?;
    let mut stuff = String::new();

    file.read_to_string(&mut stuff)?;

    let data = stuff
        .split('\n')
        .map(|x| {
            x.split("")
                .filter(|x| x.len() > 0)
                .map(|x| to_bracket(x))
                .collect::<Vec<Bracket>>()
        })
        .collect::<Vec<Vec<Bracket>>>();

    // println!("{:?}", data);
    // calc1(data);
    calc2(data);

    Ok(())
}
