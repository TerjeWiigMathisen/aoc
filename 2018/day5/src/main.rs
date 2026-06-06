//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

fn react(inp:Vec<u8>) -> i64
{
    let mut poly:Vec<u8> = inp.clone();
    let p1;
    loop {
        let mut pairs_removed:i64 = 0;
        let mut i = 1;
        while i < poly.len() {
            if poly[i] ^ poly[i-1] == 32 { // Upper/lower case pair!
                pairs_removed += 1;
                poly[i] = 32; poly[i-1] = 32;
                i += 1;
            }
            i += 1;
        }
        if pairs_removed == 0 {
            p1 = poly.len() as i64;
            break;
        }
        // Strip away the filtered pairs:
        poly.retain(|&x| x > 32);
    }
    return p1;
}

fn process(inp:&str) -> i64
{
    let poly:Vec<u8> = inp.as_bytes().to_vec().clone();
    return react(poly);
}

fn process2(inp:&str) -> i64
{
    let input_poly:Vec<u8> = inp.as_bytes().to_vec().clone();
    let mut minlen = i64::MAX;
    for letter in (b'a')..=(b'z') {
        let mut poly:Vec<u8> = input_poly.clone();
        poly.retain(|&x| x | 32 != letter);
        let l = react(poly);
        if l < minlen { minlen = l}
    }
    return minlen;
}


fn main() {
    assert!(process(
"dabAcCaCBAcCcaDA") == 10);

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { 
        process(&input); 
        process2(&input);
    }); bench_result.print_stats();

    devtime.start();
    let part1 = process(&input);
    let part2 = process2(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}