//use std::io;
//use std::env;
//use std::fs;
//use substring::Substring;
//use std::collections::HashMap;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;


fn process(startingstate:char, steps:usize) -> usize
{
    let mut tape:Vec<u8> = vec![0;2000000];
    let mut pos = tape.len()/2;
    let mut state:char = startingstate;

    for _i in 0..steps {
        match state {
            'A' => { if tape[pos] == 0 {tape[pos] = 1; pos += 1; state = 'B'} else {tape[pos] = 0; pos -= 1; state = 'E';}},
            'B' => { if tape[pos] == 0 {tape[pos] = 1; pos -= 1; state = 'C'} else {tape[pos] = 0; pos += 1; state = 'A';}},
            'C' => { if tape[pos] == 0 {tape[pos] = 1; pos -= 1; state = 'D'} else {tape[pos] = 0; pos += 1; state = 'C';}},
            'D' => { if tape[pos] == 0 {tape[pos] = 1; pos -= 1; state = 'E'} else {tape[pos] = 0; pos -= 1; state = 'F';}},
            'E' => { if tape[pos] == 0 {tape[pos] = 1; pos -= 1; state = 'A'} else {tape[pos] = 1; pos -= 1; state = 'C';}},
            'F' => { if tape[pos] == 0 {tape[pos] = 1; pos -= 1; state = 'E'} else {tape[pos] = 1; pos += 1; state = 'A';}},
            _  => println!("Bad state {state}"),
        }
    }
    let mut count = 0;
    for i in 0..tape.len() {
        count += tape[i] as usize;
    }
//    println!("Stop after {steps} steps, current state = {state} with {count} cells set");
    return count;
}

fn main() {
    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10, |_| {
        process('A',12386363);
    });
    bench_result.print_stats();

    devtime.start();
    let part1 = process('A', 12386363);
    devtime.stop();

    println!("Part1 = {part1}");
//    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}