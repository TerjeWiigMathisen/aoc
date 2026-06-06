//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
//use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

fn process(inp:String) -> (usize, usize)
{
    let mut input = inp.as_bytes().to_vec();
    let mut part1 = 0;
    for i in 0..50 {
        let mut new_input = Vec::new();
        let mut j = 1;
        while j <= input.len() {
            let mut count = 1;
            let digit = input[j-1];
            while j < input.len() && input[j] == digit {
                count += 1;
                j += 1;
            }
            if count > 9 {
                println!("{}{} ", i, count);
                new_input.push((count / 10)+b'0');
                count = count % 10;
            }
            new_input.push(count + b'0');
            new_input.push(digit);
            j += 1;
        }
        input = new_input;
        if i == 39 {
            part1 = input.len();
        }
    }
    (part1, input.len())
}

fn main() {
    let input = "1113122113".to_string();

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}