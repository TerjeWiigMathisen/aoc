//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

struct Pos {
    x:usize,
    y:usize,
}

fn trace(grid:&mut Vec<Vec<u16>>, start:Vec<Pos>, bytes:&Vec<u8>, begin:usize, end:usize)->(usize, Vec<Pos>)
{
    let mut endpos:Vec<Pos> = vec![];
    let mut i = begin;
    for p in start {
        let mut x = p.x; let mut y = p.y;
        i = begin;
        while i < end {
            let b = bytes[i];
//            println!("{}", b as char);
            i += 1;
            if b == ')' as u8 {
                endpos.push(Pos{x,y});
                return (i, endpos);
            }
            if b == '(' as u8 {
                let epos:Vec<Pos>;
                (i, epos) = trace(grid, vec![Pos{x,y}], bytes, i,end);
                for e in epos { endpos.push(e); }
            }
            else if b == '|' as u8 {
                endpos.push(Pos{x,y});
                x = p.x; y = p.y;
            }
            else if b == 'N' as u8 {y-= 1; grid[y][x] &= !1;}
            else if b == 'E' as u8 {x+= 1; grid[y][x] &= !2;}
            else if b == 'S' as u8 {grid[y][x] &= !1; y += 1;}
            else if b == 'W' as u8 {grid[y][x] &= !2; x -= 1;}
            else {panic!("Bad navigation letter:{}!", b as char)}
//            disp(grid.to_vec());
        }
    }
    return (i, endpos);
}

fn disp(grid:Vec<Vec<u16>>)
{
    let mut xmin = usize::MAX;
    let mut ymin = usize::MAX;
    let mut xmax= usize::MIN;
    let mut ymax = usize::MIN;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == u16::MAX {continue}
            if x < xmin {xmin = x;}
            if x > xmax {xmax = x;}
            if y < ymin {ymin = y;}
            if y > ymax {ymax = y;}
        }
    }
    xmax += 1; ymax += 1;
    println!("Grid from ({xmin},{ymin}) to ({xmax},{ymax})");
    for _x in xmin..xmax {
        print!("##");
    }
    println!("#");
    for y in ymin..ymax {
        for x in xmin..xmax {
            print!("{}.", if grid[y][x]&2 == 0 {'|'}else{'#'});
        }
        println!("#");
        for x in xmin..xmax {
            print!("#{}", if grid[y][x]&1 == 0 {'-'}else{'#'});
        }
        println!("#");
    }
}

const SX:usize = 500;
const SY:usize = 500;

fn process(inp:&str) -> (i64,i64)
{
    let bytes = inp.as_bytes().to_vec();
    let mut s = 0;
    while bytes[s] == '^' as u8 {s+=1;}
    let mut e = bytes.len();
    loop { e-=1; if bytes[e] == '$' as u8 {break} }

    let mut grid:Vec<Vec<u16>> = vec![vec![u16::MAX;SX*2];SY*2];
    let startpos:Pos = Pos{x:SX,y:SY};

    trace(&mut grid, vec![startpos], &bytes, s, e);
//    disp(grid.clone());
    // Run Dijkstra now!
    let mut dq:Vec<u16> = vec![500,500,0];
    let mut i = 0;
    let mut doors:u16 = 0;
    grid[SX][SY] &= 3;
    let mut p2:i64 = 0;
    while i < dq.len() {
        let (x,y,d) = (dq[i],dq[i+1],dq[i+2]);
        i += 3;
        let g = grid[y as usize][x as usize];
        if (g>>2) < (d as u16) {continue} // Have been here before!
        if d >= 1000 {p2 += 1}
        doors = d+1;
        grid[y as usize][x as usize] = (g & 3) | (d<<2) as u16;
        if g & 1 == 0 {dq.push(x); dq.push(y+1); dq.push(doors)}
        if g & 2 == 0 {dq.push(x-1); dq.push(y); dq.push(doors)}
        if grid[y as usize -1][x as usize] & 1 == 0 {dq.push(x); dq.push(y-1); dq.push(doors)}
        if grid[y as usize][x as usize +1] & 2 == 0 {dq.push(x+1); dq.push(y); dq.push(doors)}
    }

    return ((doors-1) as i64,p2);
}


fn main() {
    assert!(process(
"^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$") == (31,0));

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { process(&input); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}