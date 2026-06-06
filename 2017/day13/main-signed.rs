//use std::io;
//use std::env;
use std::fs;
//use std::collections::HashMap;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;

struct Scanner {
    col:i32,
    pos:i32,
    len:i32,
    per:i32,
}

fn forward(mut scan:&mut Scanner, steps:i32)
{
    scan.pos = new_pos(scan.pos, scan.per, steps);
}

fn backward(scan:&mut Scanner, steps:i32)
{
    forward(scan, -steps);
}

fn new_pos(pos:i32, period:i32, steps:i32) -> i32
{
    let mut npos = (pos + steps) % period;
    if npos < 0 { npos += period; }
    return npos;
}

fn process(inp:String) -> (i32,i32)
{
    let mut p1;
    let mut p2 = 0;
    let lines = inp.split("\n");

    let mut scanners:Vec<Scanner> = vec![];
    for li in lines {
        let parts:Vec<i32> = li.split(": ").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let s = Scanner {col: parts[0], pos: 0, len: parts[1], per: parts[1]*2-2, };
        scanners.push(s);
    }
    p1 = 0;

    for s in 0..scanners.len() {
        let steps = scanners[s].col;
        backward(&mut scanners[s], steps);
        if scanners[s].pos == 0 { p1 += scanners[s].col * scanners[s].len; }
    }
    scanners.sort_by(|a, b| a.len.cmp(&b.len));
    for i in 1.. {
        let mut hit:bool = false;
        for s in 0..scanners.len() {
            if new_pos(scanners[s].pos, scanners[s].per, -i) == 0 {
                hit = true;
                break;
            }
        }
        if !hit {
            p2 = i;
            break;
        }
    }
    return (p1, p2);
}

fn main() {

    assert!(process("0: 3
1: 2
4: 4
6: 4".to_owned()) == (24,10));
    let fname = "input.txt"; // instead of args[1]
    let input = fs::read_to_string(fname).expect("Error readin input file");
    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(25, |_| {
        process(input.clone());
    });
    bench_result.print_stats();

    devtime.start();
    let (part1,part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}