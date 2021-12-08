#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::File;
use std::io::Read;

// nicked
fn flatten<T>(nested: Vec<Vec<T>>) -> Vec<T> {
    nested.into_iter().flatten().collect()
}

fn is_wanted_seg(count: usize) -> usize {
    match count {
        2 => 1, // 1
        4 => 1, // 4
        3 => 1, // 3
        7 => 1, // 8
        _ => 0,
    }
}

fn calc1(stuff: Vec<Vec<&str>>) {
    let lens: Vec<usize> = flatten::<&str>(stuff)
        .into_iter()
        .map(|x| x.len())
        .collect();
    // .reduce(|acc, wires| acc + is_wanted_seg(wires))
    // .unwrap();
    let mut count = 0;
    for l in lens {
        if is_wanted_seg(l) == 1 {
            count += 1;
        }
    }
    println!("{:#?}", count);
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("ex.txt")?;
    let mut stuff = String::new();

    file.read_to_string(&mut stuff)?;
    let output_codes: Vec<Vec<&str>> = stuff
        .split('\n')
        .map(|x| x.split('|').nth(1).unwrap().split_whitespace().collect())
        .collect();

    // println!("{:?}", stuff);
    // println!("{:?}", output_codes);

    calc1(output_codes);

    // for i in 0..12 {
    //     println!("{}: {}", i, is_wanted_seg(i));
    // }

    Ok(())
}
