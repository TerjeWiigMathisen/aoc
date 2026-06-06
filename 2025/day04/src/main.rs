// Fastest run 1458800 ns

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

fn removed(stride:usize,buffer:&mut Vec<u8>)-> usize
{
    let mut remlist:Vec<usize> = Vec::new();
    let si = stride as i32;
    for p in stride+1..buffer.len()-stride-1 {
        if buffer[p] != b'@' { continue; }
        let mut neighbors = 0;
        for o in [-si-1, -si, -si+1, -1, 1, si-1, si, si+1] {
            if buffer[(p as i32 + o) as usize] == b'@' { 
                neighbors += 1; 
                //if neighbors > 3 { break; }
            }
        }
        if neighbors <= 3 { remlist.push(p); }
    }
    let ret = remlist.len();
    for p in remlist {
        buffer[p] = b'X';
    }
    ret
}

fn removed3(stride:usize,buffer:&mut Vec<u8>)-> usize
{
    let mut above:Vec<u8> = vec![0;stride];
    let mut curr:Vec<u8> = vec![0;stride];
    let mut below:Vec<u8> = vec![0;stride];
    let mut start  = stride+1;
    let end = buffer.len()-stride-1;
    let lt:&[u8] = &buffer[start-1..start+stride-2];
    let cr:&[u8] = &buffer[start..start+stride-1].clone();
    let rt:&[u8] = &buffer[start+1..start+stride].clone();
    for i in 0..stride-1 {
        curr[i] = (lt[i] == b'@') as u8 + (cr[i] == b'@') as u8 + (rt[i] == b'@') as u8;
    }
    let mut remcnt = 0;
    while start < end {
        let lt:&[u8] = &buffer[start+stride-1..start+stride+stride-2];
        let cr:&[u8] = &buffer[start+stride..start+stride+stride-1];
        let rt:&[u8] = &buffer[start+stride+1..start+stride+stride];
        for i in 0..stride-1 {
            below[i] = (lt[i] == b'@') as u8 + (cr[i] == b'@') as u8 + (rt[i] == b'@') as u8;
            if buffer[start+i] == b'@' {
                if above[i]+curr[i]+below[i] <= 4 {
                    buffer[start+i] = b'X';
                    remcnt += 1;
                } 
            }
        }
        above = curr.clone();
        curr = below.clone();
        start += stride;
    }
    remcnt
}

fn process(inp:String) -> (i32, i32)
{
    let linelen = inp.find('\n').unwrap();
    let stride = linelen+1;
    let bytes = inp.as_bytes();
    let mut buffer:Vec<u8> = Vec::with_capacity(stride*2+bytes.len()+2);
    let padding:Vec<u8> = vec![b'\n';stride+1];
    buffer.append(&mut padding.clone());
    buffer.append(&mut bytes.to_vec());
    buffer.append(&mut padding.clone());

    let part1 = removed3(stride, &mut buffer) as i32;
    let mut part2 = part1;
    loop {
        let rem = removed3(stride, &mut buffer) as i32;
        if rem == 0 { break; }
        part2 += rem;
    }
    (part1, part2)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut fname = "input.txt".to_string();
    if args.len() > 1 { fname = args[1].clone(); }

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