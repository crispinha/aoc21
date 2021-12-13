#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::File;
use std::io::Read;

type Squid = usize;
type VV<T> = Vec<Vec<T>>;
type Square = (Squid, bool);

fn all_done(map: &VV<Square>) -> bool {
    for x in 0..map.len() {
        let row = &map[x];
        for y in 0..row.len() {
            let (energy, flashed) = row[y];
            if energy > 9 && !flashed {
                return false;
            }
        }
    }
    true
}

fn legal(x: isize, y: isize, map: VV<Square>) -> bool {
    let last_x = map.len() as isize;
    if !(x >= 0 && x < last_x) {
        return false;
    }
    let last_y = map[x as usize].len() as isize;
    y >= 0 && y < last_y
}

fn step(map: &VV<Square>) -> (VV<Square>, i32) {
    let mut nu_map = map
        .into_iter()
        .map(|x| {
            x.into_iter()
                .map(|(i, _)| (i + 1, false))
                .collect::<Vec<Square>>()
        })
        .collect::<VV<Square>>();

    let mut flashes = 0;
    while !all_done(&nu_map) {
        for x in 0..nu_map.len() {
            // let mut row = &mut nu_map[x];
            for y in 0..nu_map[x].len() {
                let (energy, flashed) = nu_map[x][y];
                if energy > 9 && !flashed {
                    flashes += 1;
                    nu_map[x][y] = (energy, true);
                    let ix = x as isize;
                    let iy = y as isize;

                    if legal(ix + 1, iy, nu_map.clone()) {
                        let (nrg, flsh) = nu_map[x + 1][y];
                        nu_map[x + 1][y] = (nrg + 1, flsh);
                    }
                    if legal(ix - 1, iy, nu_map.clone()) {
                        let (nrg, flsh) = nu_map[x - 1][y];
                        nu_map[x - 1][y] = (nrg + 1, flsh);
                    }
                    if legal(ix, iy + 1, nu_map.clone()) {
                        let (nrg, flsh) = nu_map[x][y + 1];
                        nu_map[x][y + 1] = (nrg + 1, flsh);
                    }
                    if legal(ix, iy - 1, nu_map.clone()) {
                        let (nrg, flsh) = nu_map[x][y - 1];
                        nu_map[x][y - 1] = (nrg + 1, flsh);
                    }

                    if legal(ix+1, iy+1, nu_map.clone()) {
                        let (nrg, flsh) = nu_map[x+1][y+1];
                        nu_map[x+1][y+1] = (nrg + 1, flsh);
                    }
                    if legal(ix+1, iy-1, nu_map.clone()) {
                        let (nrg, flsh) = nu_map[x+1][y-1];
                        nu_map[x+1][y-1] = (nrg + 1, flsh);
                    }
                    if legal(ix-1, iy+1, nu_map.clone()) {
                        let (nrg, flsh) = nu_map[x-1][y+1];
                        nu_map[x-1][y+1] = (nrg + 1, flsh);
                    }
                    if legal(ix-1, iy-1, nu_map.clone()) {
                        let (nrg, flsh) = nu_map[x-1][y-1];
                        nu_map[x-1][y-1] = (nrg + 1, flsh);
                    }
                }
            }
        }
    }

    nu_map = nu_map
        .into_iter()
        .map(|x| {
            x.into_iter()
                .map(|(i, f)| {
                    if f {
                        return (0, false);
                    }
                    (i, f)
                })
                .collect::<Vec<Square>>()
        })
        .collect::<VV<Square>>();

    (nu_map, flashes)
}

fn calc1(map: VV<Square>) {
    let mut nu_map = map;
    let mut count = 0;
    for _ in 0..100 {
        let (m, c) = step(&nu_map);
        nu_map = m;
        count += c;
    }
    println!("{}", count);
}

fn all_0(map: &VV<Square>) -> bool {
    map.into_iter().flatten().map(|(i, _)| *i == 0).all(|x| x)
}

fn calc2(map: VV<Square>) {
    let mut nu_map = map;
    let mut count = 0;
    while !all_0(&nu_map) {
        let (m, _) = step(&nu_map);
        nu_map = m;
        count += 1;
    }
    println!("{}", count);
}


fn main() -> std::io::Result<()> {
    let mut file = File::open("ch.txt")?;
    let mut stuff = String::new();

    file.read_to_string(&mut stuff)?;

    let map = stuff
        .split('\n')
        .map(|x| {
            x.split("")
                .filter(|x| !x.is_empty())
                .map(|x| (x.parse::<Squid>().unwrap(), false))
                .collect::<Vec<Square>>()
        })
        .collect::<VV<Square>>();

    calc2(map);

    Ok(())
}
