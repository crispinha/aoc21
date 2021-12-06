use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

fn step(fishies: [u64; 9]) -> [u64; 9] {
    let mut new :[u64;9] = [0; 9];
    new[0] = fishies[1];
    new[1] = fishies[2];
    new[2] = fishies[3];
    new[3] = fishies[4];
    new[4] = fishies[5];
    new[5] = fishies[6];
    new[6] = fishies[7];
    new[7] = fishies[8];

    new[8] = fishies[0];
    new[6] += fishies[0];

    new
}

fn calc(mut fishies: [u64; 9]) {
    for _ in 0..257 {
        fishies = step(fishies);
    }

    let mut num : u64 = 0;
    for fs in &fishies {
        num += fs;
    }
    println!("{}", num);
}

fn parse_fishies(fishies: Vec<usize>) -> [u64; 9] {
    let mut fs : [u64; 9] = [0; 9];
    for fish in fishies {
        fs[fish+1] += 1;
    }
    fs
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("ch.txt")?;
    let mut stuff = String::new();

    file.read_to_string(&mut stuff)?;

    let raw_fishies: Vec<usize> = stuff.split(",").map(|x|{usize::from_str(x).unwrap()}).collect();
    let fishies = parse_fishies(raw_fishies);

    calc(fishies);

    Ok(())
}
