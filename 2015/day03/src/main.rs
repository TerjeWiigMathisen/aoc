//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

const MAXOFFS:usize = 100;

fn process(inp:String) -> (i32, i32)
{
    let mut grid:Vec<Vec<bool>> = vec![vec![false; MAXOFFS*2+1]; MAXOFFS*2+1];
    let mut x=MAXOFFS;
    let mut y=MAXOFFS;
    let mut xmin = x;
    let mut xmax = x;
    let mut ymin = y;
    let mut ymax = y;
    grid[x][y] = true;
    for b in inp.bytes()
    {
        match b
        {
            b'>' => x+=1,
            b'<' => x-=1,
            b'^' => y+=1,
            b'v' => y-=1,
            _ => {}
        }
        grid[x][y] = true;
        if x < xmin {xmin = x;}
        if x > xmax {xmax = x;}
        if y < ymin {ymin = y;}
        if y > ymax {ymax = y;}
    }
    let part1 = grid.iter().flatten().filter(|&&x| x).count() as i32;

    grid = vec![vec![false; MAXOFFS*2+1]; MAXOFFS*2+1];
    let mut x=MAXOFFS;
    let mut y=MAXOFFS;
    let mut rx=MAXOFFS;
    let mut ry=MAXOFFS;
    grid[x][y] = true;
    let bytes = inp.as_bytes();
    for i in (0..bytes.len()).step_by(2)
    {
        let b:u8 = bytes[i];
        let rb:u8 = bytes[i+1];
        match b
        {
            b'>' => x+=1,
            b'<' => x-=1,
            b'^' => y+=1,
            b'v' => y-=1,
            _ => {}
        }
        grid[x][y] = true;
        if x < xmin {xmin = x;}
        if x > xmax {xmax = x;}
        if y < ymin {ymin = y;}
        if y > ymax {ymax = y;}
        match rb
        {
            b'>' => rx+=1,
            b'<' => rx-=1,
            b'^' => ry+=1,
            b'v' => ry-=1,
            _ => {}
        }
        grid[rx][ry] = true;
        if rx < xmin {xmin = rx;}
        if rx > xmax {xmax = rx;}
        if ry < ymin {ymin = ry;}
        if ry > ymax {ymax = ry;}
    }
    let part2 = grid.iter().flatten().filter(|&&x| x).count() as i32;
    println!("({xmin},{ymin} - {xmax},{ymax} (Size = {}", (xmax-xmin+1)*(ymax-ymin+1));
    (part1,part2)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}