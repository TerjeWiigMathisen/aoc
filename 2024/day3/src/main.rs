// 7.0 us

//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

fn process(byt:&[u8]) -> (u32, u32)
{
    let mut p:usize = 0;
    //let byt: &[u8] = inp.as_bytes();
    let mut enable = u32::MAX;
    let mut part1 = 0;
    let mut part2 = 0;
    while p+7 < byt.len() {  // Minimum room for a mul(a,b)
        let c = byt[p];
        if c == b'd' {
            if byt[p+1] == b'o' {
                if byt[p+2] == b'(' && byt[p+3] == b')' {
                    enable = u32::MAX;
                    p += 3;
                }
                else if byt[p+2] == b'n' && byt[p+3] == b'\'' && byt[p+4] == b't' &&
                    byt[p+5] == b'(' && byt[p+6] == b')' {
                    enable = 0;
                    p += 6;
                }
            }
        }
        else if c == b'm' {
            if byt[p+1] == b'u' && byt[p+2] == b'l' && byt[p+3] == b'(' {
                let mut i:u32 = 0;
                p += 4;
                let mut m = p;
                while byt[m] >= b'0' && byt[m] <= b'9' {
                    i = i * 10 + (byt[m] - b'0') as u32;
                    m += 1;
                }
                if byt[m] == b',' {
                    m += 1;
                    p = m;
                    let mut j:u32 = 0;
                    while byt[m] >= b'0' && byt[m] <= b'9' {
                        j = j * 10 + (byt[m] - b'0') as u32;
                        m += 1;
                    }
                    if byt[m] == b')' {
                        if i < 1000 && j < 1000 {
                            part1 += i * j;
                            part2 += (i * j) & enable;
                        }
                        p = m;
                    }
                }
            }
        }
        p += 1;
    }
    (part1, part2)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let input = fs::read_to_string(fname).expect("Error readin input file");
    let inp = input.as_bytes();
    
    // if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}
    // The code needs at least one non-matching character at the end of the input file!

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process(inp); }); bench_result.print_stats();
    devtime.start();
    let (part1, part2) = process(inp);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} ns",devtime.time_in_nanos().unwrap());
}