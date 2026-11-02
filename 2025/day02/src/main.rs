// Fastest run 25.7 us
// HP sort 14 us
// HP u8 parser 9.6 us
// Acer 9.7 us

//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;
//use rustc_hash::FxHashMap;

fn process(inp:&str) -> (usize, usize)
{
    let mut part1 = 0;
    let mut part2 = 0;

    let tens = [10,10,100,1000,10000,100000,1000000,10000000,100000000];

    let mut seen2:Vec<usize> = Vec::with_capacity(1000);
    let mut seen1:Vec<usize> = Vec::with_capacity(1000);
    let mut prefix:[usize; 20] = [1; 20];

    let mut i = 0;
    let input = inp.as_bytes();
    loop {

        let mut fu = 0;
        let mut flen = 0;
        while input[i] != b'-' {
            fu = fu * 10 + (input[i] - b'0') as usize;
            i += 1;
            flen += 1;
            prefix[flen] = fu;
        }
        i += 1; // skip '-'
        let mut lu = 0;
        let mut llen = 0;
        while input[i] >= b'0' && input[i] <= b'9' {
            lu = lu * 10 + (input[i] - b'0') as usize;
            i += 1;
            llen += 1;
        }

//        println!("Input range: {}-{}",fu,lu);

        seen1.clear();
        seen2.clear();

        for div in 2..=llen {
            let prelen = flen / div;
            let mut preu:usize = prefix[prelen];
            let mut multiplier = tens[prelen];
            //println!("{div} {l2} {} {}",pre,multiplier);
            loop {
                let mut invalid = preu;
                for _i in 1..div {
                    invalid = invalid*multiplier+preu;
                }
                //println!("{}-{}",preu,inv);
                if invalid > lu {break;} // Outside top end of range
                if fu <= invalid { // Inside range!
                    seen2.push(invalid as usize);
                    if div == 2 { // part1
                        seen1.push(invalid as usize);
                    }
                    //println!("{}",inv);
                }
                preu += 1;
                if preu >= multiplier {
                    multiplier *= 10;
                }
            }
        }
        seen1.sort_unstable();
        seen1.dedup();
        for s in seen1.iter() {
            part1 += *s;
        }
        seen2.sort_unstable();
        seen2.dedup();
        for s in seen2.iter() {
            part2 += *s;
            //println!("{}",s);
        }
        if input[i] == b'\n' {break;}
        i += 1; // skip comma
    }
    (part1, part2)
}

fn main() {
    let fname:String = std::env::args().nth(1).unwrap_or("input.txt".into());

    let mut input = fs::read_to_string(fname).expect("Error reading input file");
    if input.as_bytes()[input.as_bytes().len()-1] != '\n' as u8 {input.push('\n');}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10000, |_| { process(&input); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}