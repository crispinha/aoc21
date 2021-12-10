#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::File;
use std::io::Read;
use crate::Tile::{Unvisited, Visited};

type Num = i32;

fn sample(x: Num, y:Num, map: &Vec<Vec<Num>>) -> Num {
    if !(x < 0 || x >= map.len() as Num) {
        let row = &map[x as usize];
        if !(y < 0 || y >= row.len() as Num) {
            return row[y as usize];
        }
    }

    Num::MAX
}

fn calc1(map: Vec<Vec<Num>>) {
    let mut count: Num = 0;
    for x in 0..map.len() {
        let row = &map[x];
        for y in 0..row.len() {
            let wx = x as Num;
            let wy = y as Num;
            let point = sample(wx, wy, &map);
            if point < sample(wx-1, wy, &map) && point < sample(wx + 1, wy, &map)
                && point < sample(wx, wy-1, &map) && point < sample(wx, wy+1, &map) {
                count += 1 + point;
            }
        }
    }

    println!("{}", count);
}

#[derive(PartialEq)]
enum Tile {
    Peak,
    Unvisited,
    Visited
}

fn floodfill(initial_x: usize, initial_y: usize, map: &mut Vec<Vec<Tile>>) -> Num {
    let mut count = 0;
    let mut working = vec![(initial_x, initial_y)];
    while !working.is_empty() {
        let (x, y) = working.pop().unwrap();
        if map[x][y] == Unvisited {

            map[x][y] = Visited;
            count += 1;

            if x > 0 {
                working.push((x - 1, y));
            }

            if x  < map.len() - 1 {
                working.push((x + 1, y));
            }

            if y > 0 {
                working.push((x, y - 1));
            }

            if y  < map[x].len() - 1 {
                working.push((x, y + 1));
            }
        }
    }
    count
}

fn calc2(in_map:Vec<Vec<Num>>) {
    let mut map = in_map.into_iter()
                                        .map(|x| x.into_iter()
                                            .map(|x| match x {9 => Tile::Peak, _ => Tile::Unvisited})
                                            .collect::<Vec<Tile>>())
                                        .collect::<Vec<Vec<Tile>>>();

    let mut sizes: Vec<Num> = vec![];
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if map[x][y] == Unvisited {
                let size = floodfill(x, y, &mut map);
                sizes.push(size);
            }
        }
    }
    sizes.sort();
    let res = sizes.into_iter().rev().take(3).reduce(|acc, x| acc * x).unwrap();
    println!("{}", res);
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("ch.txt")?;
    let mut stuff = String::new();

    file.read_to_string(&mut stuff)?;

    let map: Vec<Vec<Num>> = stuff.split('\n')
                                  .map(|x|
                                      x.split("")
                                       .filter(|x| x.len() > 0)
                                       .map(|x|x.parse::<Num>().unwrap())
                                       .collect::<Vec<Num>>())
                                  .filter(|x| x.len() > 0)
                                  .collect();

    // println!("{:?}", map);
    // calc1(map);
    calc2(map);
    Ok(())

}
