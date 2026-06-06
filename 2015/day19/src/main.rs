//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

fn process(inp:String) -> (i32, i32)
{
    let mut floor = -1;
    let mut first_basement = 0;
    let mut chars = 0;
    for c in inp.as_bytes() {
        chars += 1;
        let d = *c as u32;
        floor += (d+d+(0xffffffff-80)) as i32;
         if floor == 0 && first_basement == 0 {
            first_basement = chars;
        }
    }
    (-floor-1, first_basement)
}
/*         
fn process(inp:String) -> (i64, i64)
{
    let mut floor = 0;
    let mut first_basement = 0;
    let mut chars = 0;
    for c in inp.as_bytes() {
        chars += 1;
        let d = 81 - ((c*2) as i64); // '('->1, ')'->-1
        floor += d;
        if floor == -1 && first_basement == 0 {
            first_basement = chars;
        }
        match c
        {
            b'(' => floor += 1,
            b')' => {
                floor -= 1;
                if floor == -1 && first_basement == 0 {
                    first_basement = chars;
                }
            },
            _ => panic!("Invalid character")
        }
    }
    (floor, first_basement)
}
*/

fn main() {
    let args:Vec<String> = env::args().collect();
    let fname = if args.len() > 1 { args[1].clone() } else { "input.txt".to_string() };
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