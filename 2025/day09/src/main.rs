// Fastest run 165 us

use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;
use rustc_hash::FxHashMap;

struct Point {
    x:u32,
    y:u32,
}

fn process(inp:&str) -> (i32, i32)
{
    let mut points:Vec<Point> = Vec::new();
    for line in inp.lines() {
        let pair = line.split_once(',').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        points.push(Point{x:pair[0], y:pair[1]});
    }
    let mut part1 = 0;
    let mut part2 = 0;
    (part1, part2)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}