// Fastest run Acer 7.0 us
// Surface 21.8 us

//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

pub fn process(byt:&[u8]) -> (u32, u32)
{
    let mut p:usize = 0;
    //let byt: &[u8] = inp.as_bytes();
    let mut enable = u32::MAX;
    let mut part1 = 0;
    let mut part2 = 0;
    while p+7 < byt.len() {  // Minimum room for a mul(a,b)
        let c = byt[p];
        let c4 = u32::from_le_bytes(byt[p..p+4].try_into().unwrap());
        if c4 == (b'd' as u32 + (b'o' as u32)*256 + 
            (b'(' as u32)*65536 + (b')' as u32)*256*65536) { 
            enable = u32::MAX;
            p += 4;
        }
        else if c4 == (b'd' as u32 + (b'o' as u32)*256 + 
              (b'\'' as u32)*65536 + (b't' as u32)*256*65536) &&
               byt[p+5] == b'(' && byt[p+6] == b')' {
            enable = 0;
            p += 6;
        }
        else if c4 == (b'm' as u32 + (b'u' as u32)*256 + 
              (b'l' as u32)*65536 + (b'(' as u32)*256*65536) {
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