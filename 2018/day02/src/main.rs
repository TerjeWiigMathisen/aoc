// Fastest run: 11.5 us
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;


fn process(inp:&str) -> i64
{
    let mut cnt2 = 0;
    let mut cnt3 = 0;
    let lines = inp.split("\n");
    for li in lines {
        let bytes = li.as_bytes();
        let mut cnt:[u8;32] = [0;32];
        //let mut cnt:Vec<i32> = vec![0;26];
        for i in 0..bytes.len() {
            let c = bytes[i] & 31;
            cnt[c as usize] += 1;
        }
        let inc2 = cnt.contains(&2);
        let inc3 = cnt.contains(&3);
        // //let mut inc2 = 0;
        // let mut inc3 = 0;
        // for i in 0..26 {
        //     if cnt[i] == 2 {inc2 = 1}
        //     else if cnt[i] == 3 {inc3 = 1}
        // }
        // for i in 0..26 {
        //     if cnt[i] == 2 {inc2 = 1}
        //     else if cnt[i] == 3 {inc3 = 1}
        // }
        cnt2 += inc2 as i64;
        cnt3 += inc3 as i64;
    }
    return cnt2 * cnt3;
}

fn process2(inp:&str) -> String
{
    let lines:Vec<&str> = inp.lines().collect();
    let mut eq:String = "".to_owned();
    for j in 1..lines.len() {
        for i in 0..j {
            let f = lines[i].as_bytes();
            let s = lines[j].as_bytes();
            let mut diff = 0;
            for n in 0..f.len() {
                if f[n] != s[n] { 
                    diff += 1;
                    if diff > 1 {break;}
                }
            }
            if diff == 1 {
                for n in 0..f.len() {
                    if f[n] == s[n] { eq.push(f[n] as char)}
                }
                return eq;
            }
        }
    }
    return eq;
}


fn main() {

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { 
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