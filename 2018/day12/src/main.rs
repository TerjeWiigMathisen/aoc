// Fastest run: 929.1 us

//use std::collections::HashMap;
//use std::str;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;
use fxhash::FxHashMap;

fn process(inp:&str, generations:usize) -> (i64,i64)
{
    let lines:Vec<String> = inp.split("\n").map(|s| s.to_string()).collect();
    let mut nextgen:FxHashMap<Vec<u8>,u8> = FxHashMap::default();
    let mut current_state:Vec<u8> = lines[0].as_bytes().to_owned();
    let mut pad = 0;
    for prefix in 0.. {
        if current_state[prefix] == '#' as u8 {
            pad = prefix;
            break;
        }
        current_state[prefix] = '.' as u8;
    }
    for _ in 0..120 {
        current_state.push('.' as u8);
    }
    //println!("{} - {}", 0, str::from_utf8(&current_state).unwrap());
    for l in 2..lines.len() {
        let li = lines[l].as_bytes();
        let key:Vec<u8> = li[0..5].to_vec();
        let r = li[9];
        nextgen.insert(key, r);
    }
    let mut p1 = 0;
    let mut p2 = 0;
    for gen in 1..=generations {
        let mut next_state:Vec<u8> = vec!['.' as u8;current_state.len()];
        for x in 0..current_state.len()-4 {
            let sample:Vec<u8> = current_state[x..(x+5)].to_vec();
            let n:&u8 = nextgen.get(&sample).unwrap_or(&('.' as u8));
            next_state[x+2] = *n;
        }
        current_state = next_state;
        p2 = 0; p1 = 0;
        for i in 0..current_state.len() {
            if current_state[i] == '#' as u8 {
                p1 += i as i64 - pad as i64;
                if p2 <= 0 {p2 = i as i64 - pad as i64;}
            }
        }
//        println!("{gen:3} - {p1:6} - {}", str::from_utf8(&current_state).unwrap());
        if gen == 100 {
            p1 += (generations-gen) as i64 *57;
            break;
        }
    }
    return (p1, p2);
}


fn main() {
    assert!(process(
"initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #",20).0 == 325);

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { 
        process(&input,20);
        process(&input,50000000000);
     }); bench_result.print_stats();

    devtime.start();
    let (part1, _p1) = process(&input,20);
    let (part2, _p2) = process(&input,50000000000);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}