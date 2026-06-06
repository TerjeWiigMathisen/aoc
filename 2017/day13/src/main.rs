//use std::io;
//use std::env;
use std::fs;
//use std::collections::HashMap;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;

struct Scanner {
    depth:u32,
    position:u32,
    len:u32,
    period:u32,
}

fn new_pos(position:u32, period:u32, steps:u32) -> u32
{
    return (position + steps) % period;
}

const MAXMUL:u32 = 16;
const LCM:u32 = 240*14; // Lowest common multiplier for 2,4,6,8,10,12,14,16

fn process(inp:String) -> (u32,u32)
{
    let lines = inp.split("\n");

    let mut scanners:Vec<Scanner> = vec![];
    for li in lines {
        let parts:Vec<u32> = li.split(": ").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let s = Scanner {depth: parts[0], position: 0, len: parts[1], period: parts[1]*2-2,};
        scanners.push(s);
    }

    let mut p1 = 0;
    for s in 0..scanners.len() {
        scanners[s].position = new_pos(0, scanners[s].period, scanners[s].depth);
        if scanners[s].position == 0 { p1 += scanners[s].depth * scanners[s].len; }
    }
    scanners.sort_by(|a, b| a.len.cmp(&b.len));

    let mut mask:Vec<bool> = vec![false;LCM as usize];
    let mut starting_scanner = 0;
    for s in 0..scanners.len() {
        if scanners[s].period > MAXMUL { 
            starting_scanner = s;
            break; 
        }
        for i in 0..LCM {
            if new_pos(scanners[s].position, scanners[s].period,i)== 0 {
                mask[i as usize] = true;
            }
        }
    } 
    let mut block = 0;
    loop {
'outer: for i in 0..LCM {
            if mask[i as usize] { continue; }
            for s in starting_scanner..scanners.len() {
                if new_pos(scanners[s].position, scanners[s].period, i+block) == 0 {
                    continue 'outer;
                }
            }
            return (p1,i+block);
        }
        block += LCM;
    }
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