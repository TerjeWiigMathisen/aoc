use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;


fn process(inp:String) -> i64
{
//    let lines = inp.split("\n");
//    if lines.len() > 0 && lines[lines.len()-1].len() == 0 {
//        lines.pop();
//    }

    return inp.split("\n").
    map(|x| if x.len() > 0 {x.parse::<i64>().unwrap()} else {0}).
    sum();
}

fn process2(inp:String) -> i64
{
    let delta:Vec<i64> = inp.split("\n").
    map(|x| if x.len() > 0 {x.parse::<i64>().unwrap()} else {0}).collect();
    let mut i = 0;
    let mut f = 0;
    let mut hm:HashMap<i64,usize>=HashMap::new();
    hm.insert(0,1);
    loop {
        f += delta[i];
        if hm.contains_key(&f) {
            println!("Found {f} at {i} hm = {}",hm.len());
            return f;
        }
        hm.insert(f,1);
        i += 1;
        if i >= delta.len() { i = 0;}
    }
}


fn main() {

    let fname = "input.txt"; // instead of args[1]
    let input = fs::read_to_string(fname).expect("Error readin input file");
    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let part1 = process(input.clone());
    let part2 = process2(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}