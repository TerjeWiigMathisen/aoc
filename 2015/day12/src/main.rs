//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;
use serde_json::Value;

fn process(inp:String) -> (i64,i64)
{
    let mut part1 = 0;
    let mut part2 = 0;

    let mut red = 0;

    let v: Value = serde_json::from_str(&inp).unwrap();
    let red_begin = Value::String("red_begin".to_string());
    let red_end = Value::String("red_end".to_string());
    let mut stack = Vec::new();
    stack.push(v);
    while !stack.is_empty()
    {
        let current = stack.pop().unwrap();
        match current
        {
            Value::Object(map) => {
                let mut red = false;
                for (_k, v) in map.clone()
                {
                    if v == Value::String("red".to_string()) { red = true; }
                }
                if red { stack.push(red_begin.clone()); }
                for (_k, v) in map
                {
                    stack.push(v);
                }
                if red { stack.push(red_end.clone()); }
            },
            Value::Array(arr) => {
                for v in arr
                {
                    stack.push(v);
                }
            },
            Value::Number(n) => {
                part1 += n.as_i64().unwrap();
                if red == 0 { part2 += n.as_i64().unwrap(); }
            },
            Value::String(s) => {
                if s == "red_begin" { red += 1; }
                if s == "red_end" { red -= 1; }
            },
            _ => {}
        }
    }

    (part1, part2)

}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}