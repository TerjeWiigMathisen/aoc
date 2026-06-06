//use std::io;
//use std::env;
use std::fs;
use std::collections::HashMap;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;

fn process(inp:String) -> (i32,i32)
{
    let mut p1 = 0;
    let p2;
    let mut input = inp.clone();
    while input.as_bytes()[input.len()-1] <= ' ' as u8 {
        input.pop();
    }
    let mut lines:Vec<&str> = input.split("\t").collect();
    if lines.len() > 0 && lines[lines.len()-1].len() == 0 { lines.pop(); }

    let mut b1:Vec<usize> = lines.iter().map(|x| x.parse::<usize>().unwrap()).collect();
    let mut seen:HashMap<String,i32> = HashMap::new();
    let blen = b1.len();
    loop {
        let key :String = format!("{:?}",b1);
        if seen.contains_key(&key) { 
            p2 = p1 - seen[&key];
            break;
        }
        seen.insert(key,p1);
        p1 += 1;
        // Re-distribute:
        let mut maxi = 0;
        let mut max = b1[0];
        for i in 1..blen {
            if b1[i] > max {
                maxi = i;
                max = b1[i];
            }
        }
        let blocks = max;
        b1[maxi] = 0;
        let all = blocks / blen;
        for i in 0..blen { b1[i] += all;}
        let mut rem = blocks - all * blen;
        let mut j = maxi;
        while rem > 0 {
            j += 1;
            if j >= blen { j = 0;}
            b1[j] += 1;
            rem -= 1;
        }
    }
    return (p1,p2);
}

fn main() {

    assert!(process("0\t2\t7\t0".to_owned()) == (5,4) );
    let fname = "input.txt"; // instead of args[1]
    let input = fs::read_to_string(fname).expect("Error readin input file");
    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(25, |_| {
        process(input.clone());
    });
    bench_result.print_stats();

    devtime.start();
    let (part1,part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}