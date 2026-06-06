//use std::io;
//use std::env;
use std::fs;
//use std::collections::HashMap;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;

fn process(inp:String) -> (i32,i32)
{
    let mut p1 = 0;
    let mut p2 = 0;
    let mut lines:Vec<&str> = inp.split("\n").collect();
    if lines.len() > 0 && lines[lines.len()-1].len() == 0 {
        lines.pop();
    }

    let mut offs:Vec<i32> = lines.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut o2 = offs.clone();
    let mut ip:i32 = 0;
    loop {
        if ip < 0 || ip >= offs.len() as i32 { break; }
        let o = offs[ip as usize];
        offs[ip as usize] = o+1;
        ip += o;
        p1 += 1;
    }
    ip = 0;
    loop {
        if ip < 0 || ip >= o2.len() as i32 { break; }
        let mut o = o2[ip as usize];
        let nextip = ip + o;
        if o >= 3 { o -= 1; }
        else { o += 1;}
        o2[ip as usize] = o;
        ip = nextip;
        p2 += 1;
    }
    return (p1,p2 & -1);
}

fn main() {

    assert!(process("0
3
0
1
-3".to_owned()) == (5,10) );
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