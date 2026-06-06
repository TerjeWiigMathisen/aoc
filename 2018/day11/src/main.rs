//use std::collections::HashMap;
//use std::io;
//use std::env;
//use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

fn process(serial:u32,gridmin:u32, gridmax:u32) -> (i32,i32,i32)
{
    let mut grid:Vec<Vec<i8>> = vec![vec![0;301];301];
    for y in 1..=300 {
        for x in 1..=300 {
            let rack_id = x as u32 + 10;
            let mut power = rack_id * y as u32;
            power += serial;
            power *= rack_id;
            power /= 100;
            power %= 10;
            grid[y][x] = (power as i8) - 5;
        }
    }
    let mut smax = 0;
    let mut xmax = 0;
    let mut ymax = 0;
    let mut gmax = gridmin;
    for g in gridmin..=gridmax {
        let mut linesum:Vec<i32> = vec![0;301];
        for y in 1..=(301-g as usize) {
            let mut lsum = 0;
            for j in 0..g as usize {
                lsum += grid[y][1+j] as i32;
            }
            linesum[y] = lsum;
        }
        for x in 2..=(301-g as usize) {
            for y in 1..=(301-g as usize) {
                linesum[y] += grid[y][x + g as usize - 1] as i32 - 
                        grid[y][x-1] as i32;
            }

            for y in 1..=(301-g as usize) {
                let mut sum:i32 = 0;
                for i in 0..g as usize {
                    sum += linesum[y+i];
                }
                if sum > smax {
                    xmax = x;
                    ymax = y;
                    smax = sum;
                    gmax = g;
//                    println!("{x},{y},{g} -> {sum}");
                }
            }
        }
        if g > gmax+2 {break;}
    }
    return (xmax as i32,ymax as i32,gmax as i32);
}

fn main() {
    assert!(process(18,3,3) == (33,45,3));
    assert!(process(18,3,30) == (90,269,16));

    let input = 5153;

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(25, |_| { 
        process(input,3,3); 
        process(input,1,300);
    }); bench_result.print_stats();

    devtime.start();
    let (p1, p2, _gsize) = process(input,3,3);
    let (part1, part2, gsize) = process(input,3,300);
    devtime.stop();

    println!("Part1 = {p1},{p2}");
    println!("Part2 = {part1},{part2},{gsize}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}