// Fastest run 22861us (22.861ms)

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
    let mut part1 = 0;
    let mut part2 = 0;
    let lines = inp.split("\n").collect::<Vec<&str>>();
    let mut lights = vec![vec![0; 1000]; 1000];
    let mut lights2 = vec![vec![0; 1000]; 1000];
    for line in lines
    {
        let parts = line.split(" ").collect::<Vec<&str>>();
        if parts.len() == 4 // toggle
        {
            let start = parts[1].split(",").collect::<Vec<&str>>();
            let end = parts[3].split(",").collect::<Vec<&str>>();
            let startx = start[0].parse::<usize>().unwrap();
            let starty = start[1].parse::<usize>().unwrap();
            let endx = end[0].parse::<usize>().unwrap();
            let endy = end[1].parse::<usize>().unwrap();
            for x in startx..=endx
            {
                for y in starty..=endy
                {
                    lights[x][y] ^= 1;
                    lights2[x][y] += 2;
                }
            }
        }
        else if parts.len() == 5 // turn on/off
        {
            let start = parts[2].split(",").collect::<Vec<&str>>();
            let end = parts[4].split(",").collect::<Vec<&str>>();
            let startx = start[0].parse::<usize>().unwrap();
            let starty = start[1].parse::<usize>().unwrap();
            let endx = end[0].parse::<usize>().unwrap();
            let endy = end[1].parse::<usize>().unwrap();
            if parts[1] == "on" {
                for x in startx..=endx
                {
                    for y in starty..=endy
                    {
                        lights[x][y] = 1;
                        lights2[x][y] += 1;
                    }
                }
            }
            else { // "off"
                for x in startx..=endx
                {
                    for y in starty..=endy
                    {
                        lights[x][y] = 0;
                        let l2 = lights2[x][y];
                        lights2[x][y] = if l2 > 0 {l2-1} else {0};
                    }
                }

            }
        }
    }
    for x in 0..1000
    {
        for y in 0..1000
        {
            part1 += lights[x][y];
            part2 += lights2[x][y];
        }
    }
    (part1,part2)
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