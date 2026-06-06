// Fastest run: 2.7388 ms

//use std::str::from_utf8;

//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
//use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

fn process(inp:String) -> String
{
    let mut input = inp.as_bytes().to_vec();
    loop {
        let mut i = input.len() - 1;
        loop
        {
            let mut c = input[i]+1;
            if c > b'z'
            {
                input[i] = b'a';
                i -= 1;
            }
            else
            {
                if c == b'i' || c == b'o' || c == b'l' {
                    c += 1;
                    for j in i+1..input.len() {
                        input[j] = b'a';
                    }
                }
                input[i] = c;
                break;
            }
        }
//        println!("{}", from_utf8(&input).unwrap());
        // Check validity:
        let mut has_straight = false;
        for k in 2..input.len()
        {
            let ii = input[k-2];
            if ii + 1 == input[k-1] && ii + 2 == input[k]
            {
                has_straight = true;
                break;
            }
        }
        if !has_straight { continue; }
    //    if input.iter().any(|&x| x == b'i' || x == b'o' || x == b'l') { continue; }
        for k in 0..input.len()-3
        {
            let ii = input[k];
            if ii == input[k+1]
            {
                for j in k+2..input.len()-1
                {
                    let jj = input[j];
                    if ii != jj && jj == input[j+1]
                    {
                        return input.to_vec().into_iter().map(|x| x as char).collect();
                    }
                }
            }
        }
    }
}

fn main() {
    let input = "hxbxwxba".to_string();

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { 
        process(process(input.clone())); }); bench_result.print_stats();

    devtime.start();
    let part1 = process(input.clone());
    let part2 = process(part1.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}