// Fastest run 248.1 us

//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;
use rustc_hash::FxHashMap;

fn ways(line:usize, pos:usize, lines:&Vec<&str>, cache:&mut FxHashMap<u64,u64>) -> u64
{
    let key = line as u64 | ((pos as u64) << 32);
    if cache.contains_key(&key) { return *cache.get(&key).unwrap(); }
    if line >= lines.len() { return 1;}

    if lines[line].as_bytes()[pos] == b'^' {
        let w = ways(line+1,pos-1, lines, cache) + ways(line+1,pos+1, lines, cache);
        cache.insert(key, w);
        return w;
    }
    let w = ways(line+1,pos, lines, cache);
    cache.insert(key, w);
    w
}

fn process(inp:String) -> (u64, u64)
{
    let lines = inp.lines().collect::<Vec<&str>>();
    let start = lines[0].find('S').unwrap();
    let mut tach:FxHashMap<u8,u8> = FxHashMap::default();
    let mut part1 = 0;
    //let mut part2 = 0;
    tach.insert(start as u8, 0);
    for line in inp.lines().skip(1) {
        let mut splits = 0;
        let mut ntach:FxHashMap<u8,u8> = FxHashMap::default();
        let tkeys = tach.keys().collect::<Vec<&u8>>();
        for tkey in tkeys {
            let tk = *tkey as usize;
            if line.as_bytes()[tk] == b'^' {
                let (p,n) = (tk-1,tk+1);
                ntach.entry(p as u8).and_modify(|e| *e += 1).or_insert(1);
                ntach.entry(n as u8).and_modify(|e| *e += 1).or_insert(1);
                splits += 1;
            }
            else {
                ntach.entry(tk as u8).and_modify(|e| *e += 1).or_insert(1);
            }
        }
        tach = ntach;
        part1 += splits;

    }
    let mut cache:FxHashMap<u64,u64> = FxHashMap::default();
    let part2 = ways(1, start, &lines, &mut cache);
    (part1, part2)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut fname = "input.txt".to_string();
    if args.len() > 1 { fname = args[1].clone(); }

    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}