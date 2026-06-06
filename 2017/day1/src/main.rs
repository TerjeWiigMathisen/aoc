//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;

fn process(inp:String) -> (usize,usize)
{
    let mut p1 = 0;
    let mut p2 = 0;

    let mut input = inp;
    let mut pos = input.len();
    while pos > 0 && input.as_bytes()[pos-1] <= 32 {
        input.pop();
        pos -= 1;
    }
    assert!(pos & 1 == 0);
    let mut prev= input.as_bytes()[pos-1];
    for curr in input.as_bytes() {
        if *curr < '0' as u8 {break; }
        if *curr == prev { p1 += (prev & 15) as usize;}
        prev = *curr;
    }
    let phalf = pos >> 1;
    for i in 0..phalf {
        if input.as_bytes()[i] == input.as_bytes()[i+phalf] { 
            p2 += (input.as_bytes()[i] & 15) as usize; 
        }
    }
    return (p1,p2*2);
}

fn main() {
    //let args: Vec<String> = env::args().collect();

//    let test = "91212129".to_string();

//    assert!(process(test) == (9,0));
//    assert!(process("1122".to_string()) == (3,0));
//    assert!(process("1111".to_string()) == (4,0));
//    assert!(process("1234".to_string()) == (0,0));

    let fname = "input.txt"; // instead of args[1]
    let input = fs::read_to_string(fname).expect("Error readin input file");
    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| {
        process(input.clone());
    });
    bench_result.print_stats();

    devtime.start();
    let (part1,part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} ns",devtime.time_in_nanos().unwrap());
}