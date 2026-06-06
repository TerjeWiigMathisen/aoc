// Fastest run (static/GLOBAL VARS): 26.6 us

//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
use std::u32;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

static mut SHORTEST_COMBINATION:usize = usize::MAX;
static mut SHORTEST_COUNT:u32 = 0;
static mut CONTAINERS:Vec<u8> = Vec::new();
static mut NROFCONTAINERS:usize = 0;
// static mut COMBINATIONS:[u32;16] = [0;16];

fn fill_static(currentsum:u8, containersused:usize, startindex:usize) -> u32 {
    let mut count = 0;
    let used = containersused+1;
    unsafe {
        for i in startindex..NROFCONTAINERS {
            let csize = CONTAINERS[i];
            let sum = currentsum + csize;
            if sum > 150 {
                break;
            }
            if sum == 150 {
                count += 1;
                // COMBINATIONS[used] += 1;
                if used < SHORTEST_COMBINATION {
                    SHORTEST_COMBINATION = used;
                    SHORTEST_COUNT = 1;
                } else if used == SHORTEST_COMBINATION {
                    SHORTEST_COUNT += 1;
                }
                continue;
            }
            // if sum < 150 {
            count += fill_static(sum, used, i+1);
            //}
        }
    }
    count
}

fn process_static(inp:String) -> (u32, u32)
{
    unsafe {
        let mut containers:Vec<u8> = inp.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
        containers.sort_unstable();
        NROFCONTAINERS = containers.len();
        CONTAINERS = containers;
        SHORTEST_COMBINATION = usize::MAX;
        SHORTEST_COUNT = 0;

        let part1 = fill_static(0, 0, 0);
        // let mut part2 = 0;
        // for c in 0..16 {
        //     part2 = COMBINATIONS[c];
        //     if part2 > 0 { break; }
        // }
        let part2 = SHORTEST_COUNT;

        (part1, part2)
    }
}


fn fill(currentsum:u8, containersused:u32, containers:&[u8], shortest_combination:u32, shortest_count:u32) -> (u32, u32, u32) {
    let mut count = 0;
    let used = containersused+1;
    let mut shortest_combination = shortest_combination;
    let mut shortest_count = shortest_count;

    for i in 0..containers.len() {
        let csize = containers[i];
        let sum = currentsum + csize;
        if sum > 150 {
            break;
        }
        if sum == 150 {
            count += 1;
            if used < shortest_combination {
                shortest_combination = used;
                shortest_count = 1;
            } else if used == shortest_combination {
                shortest_count += 1;
            }
            continue;
        }
        // if sum < 150 {
        let (c, sh, sh_count) = fill(sum, used, &containers[i+1..], shortest_combination, shortest_count);
        count += c;
        shortest_combination = sh;
        shortest_count = sh_count;
    }
    (count, shortest_combination, shortest_count)

}

fn process(inp:String) -> (u32, u32)
{
    let mut containers:Vec<u8> = inp.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
    containers.sort_unstable();
    let (part1, _sh_comb, part2) = fill(0, 0, &containers, u32::MAX, 0);

    (part1, part2)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process_static(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process_static(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}