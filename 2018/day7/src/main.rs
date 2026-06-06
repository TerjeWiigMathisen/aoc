//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

struct Dependencies {
    before:Vec<u32>,
    follows:Vec<u32>,
}

struct Bf {
    ff:char,
    bb:char,
}

fn process(inp:String) -> String
{
    let lines:Vec<String> = inp.split("\n").map(|s| s.to_string()).collect();
    let p = 
      parser!("Step " b:alpha " must be finished before step " f:alpha " can begin." => 
        Bf{bb:b, ff:f});
    let mut dep:Dependencies = Dependencies{before:vec![0;26],follows:vec![0;26]};
    let mut seen:Vec<bool>=vec![false;26];
    for li in lines {
        let bf:Bf = p.parse(&li).unwrap();
        let b = (bf.bb as u8 - 'A' as u8) as usize;
        assert!(b as u8 <= 'Z' as u8);
        let f = (bf.ff as u8 - 'A' as u8) as usize;
        assert!(f as u8 <= 'Z' as u8);
        dep.before[f] |= 1 << b;
//        dep.follows[b] |= 1 << f;
        seen[f] = true;
        seen[b] = true;
    }
    let mut p1 = String::from("");
    'outer: loop {
        for fc in 0..26 {
            if !seen[fc] {continue}
            if dep.before[fc] == 0 {
                p1.push((fc as u8+'A' as u8) as char);
                for bc in 0..26 {
                    dep.before[bc] &= !(1 << fc);
                }
                dep.before[fc] = u32::MAX;
                continue 'outer;
            }
        }
        break;
    }
    return p1;
}

fn process2(inp:String, workers:usize, steptime:u32) -> i64
{
    let lines:Vec<String> = inp.split("\n").map(|s| s.to_string()).collect();
    let p = 
      parser!("Step " b:alpha " must be finished before step " f:alpha " can begin." => 
        Bf{bb:b, ff:f});
    let mut dep:Dependencies = Dependencies{before:vec![0;26],follows:vec![0;26]};
    let mut seen:Vec<bool>=vec![false;26];
    for li in lines {
        let bf:Bf = p.parse(&li).unwrap();
        let b = (bf.bb as u8 - 'A' as u8) as usize;
        assert!(b as u8 <= 'Z' as u8);
        let f = (bf.ff as u8 - 'A' as u8) as usize;
        assert!(f as u8 <= 'Z' as u8);
        dep.before[f] |= 1 << b;
        dep.follows[b] |= 1 << f;
        seen[f] = true;
        seen[b] = true;
    }
    let mut p2 = 0;
    let mut working:Vec<u32> = vec![0;workers];
    let mut working_on:Vec<usize> = vec![26;workers];
    let mut idle = workers;
    let mut jobs_to_do:usize = seen.iter().map(|x| if *x {1} else {0}).sum();
    loop {
        // Start by finding pairs of idle workers and jobs that can be done
        while idle > 0 {
            // Find first free worker:
            let mut w = 0;
            loop {
                if working[w] == 0 { break; }
                w += 1;
            }
            // Find a job he can work on?
            let mut job = usize::MAX;
            for fc in 0..26 {
                if !seen[fc] {continue}
                if dep.before[fc] == 0 {
                    job = fc;
                    dep.before[fc] = u32::MAX; // Mark as unavailable
                    break;
                }
            }
            if job >= 26 { break} // No more jobs found
        
            working_on[w] = job;
            working[w] = steptime + job as u32 + 1;
            idle -= 1;
        }

        // Did we have anything more to do?
        if jobs_to_do == 0 {return p2;}

        // Now increment the simulation time and count down all active jobs until at least one is done
        loop {
            let mut freed = false;
//            print!("time = {p2} ");
            for w in 0..working.len() {
//                print!(" {} ", if working[w] > 0 {(working_on[w] as u8 + 'A' as u8) as char} else {'.' as char});
                if working[w] > 0 {
                    working[w] -= 1;
                    if working[w] == 0 { // Work done, free up resources:
                        jobs_to_do -= 1;
                        let fc = working_on[w];
                        for bc in 0..26 {
                            dep.before[bc] &= !(1 << fc);
                        }
                        dep.before[fc] = u32::MAX;
                        idle += 1;
                        freed = true;
                    }
                }
            }
//            println!("");
            p2 += 1;
            if freed {break}
        }
        if jobs_to_do == 0 {return p2;}
    }
}


fn main() {
    assert!(process2(
"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.".to_owned(),2,0) == 15);

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let part1 = process(input.clone());
    let part2 = process2(input.clone(),5,60);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}