// Fastest run 45.3 us

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

fn _recursive_jolt(first:usize, digits:usize, n:Vec<u8>, res:&mut Vec<u8>) -> u64
{
	let digits = digits - 1;
	let mut max:u8 = 0;
	let mut pos:usize = 0;
	for p in first..(n.len()-digits) {
		if n[p] > max {
			max = n[p];
			pos = p;
            if max == b'0' as u8 {
                break;
            }
		}
	}
    res.push(max);
	if digits > 0 {
		_recursive_jolt(pos+1,digits, n, res);
	}
    0
}

fn _recursive_joltu64(first:usize, digits:usize, n:Vec<u8>, res:u64) -> u64
{
	let digits = digits - 1;
	let mut max:u8 = 0;
	let mut pos:usize = 0;
	for p in first..(n.len()-digits) {
		if n[p] > max {
			max = n[p];
			pos = p;
		}
	}
    let mut res = res*10 + max as u64;
	if digits > 0 {
		res = _recursive_joltu64(pos+1, digits, n, res);
	}
    res
}

fn _jolt(first:usize, digits:usize, n:Vec<u8>, res:&mut Vec<u8>) -> u64
{
	let mut digits = digits-1;
    let mut first = first;
    loop {
        let mut max:u8 = 0;
        let mut pos:usize = 0;
        for p in first..(n.len()-digits) {
            if n[p] > max {
                max = n[p];
                pos = p;
            }
        }
        res.push(max);
        first = pos+1;
        if digits ==  0 {
            break;
        }
        digits -= 1;
    }
    0
}

#[cold]
fn update(np:u64, pos:&mut usize, p:usize) -> u64
{
    *pos = p;
    np
}

fn joltu64(digits:usize, n:&[u8]) -> u64
{
	let mut digits = digits;
    let mut first = 0;
    let mut res = 0;
    let nlen = n.len();
    loop {
        digits -= 1;
        let mut max:u64 = 0;
        let mut pos:usize = first;
        for p in first..nlen-digits {
            let np = n[p] as u64;
            if np > max {
                        // max = n[p] as u64;
                        // pos = p;
                max = update(np, &mut pos, p);
                // if max == b'0' as u64 {
                //     break;
                // }
            }
        }
        res = res*10 + max - b'0' as u64;
        // println!("{res}");
        first = pos+1;
        if digits ==  0 {
            break;
        }
    }
    res
}

fn _jolt2(digits:usize, n:&[u8]) -> u64
{
	let mut digits = digits;
    let mut first = 0;
    let mut res = 0;
    let nlen = n.len();
    let mut nextmax = 0;
    let mut nextpos = 0;
    loop {
        digits -= 1;
        let mut max:u64 = 0;
        let mut pos:usize = 0;
        for p in first..nlen-digits {
            let np = n[p] as u64;
            if np >= max {
                if np > max {
                    max = np;
                    pos = p;
                    nextmax = 0;
                } else if np > nextmax {
                    nextmax = np;
                    nextpos = p;
                }
            }
            if max == b'9' as u64 {
                break;
            }
        }
        res = res*10 + max - b'0' as u64;
        // println!("{res}");
        first = pos+1;
        if nextmax > 0 {
            first = nextpos
        }

        if digits ==  0 {
            break;
        }
    }
    res
}

fn _process(inp:String) -> (u64,u64)
{
    let mut part1 = 0;
    let mut part2 = 0;
    for line in inp.lines() {
        let mut res:Vec<u8> = Vec::new();
        _recursive_jolt(0,2, line.as_bytes().to_vec(), &mut res);
        part1 += String::from_utf8(res.clone()).unwrap().parse::<u64>().unwrap();
        //println!("{} -> {}", line, String::from_utf8(res).unwrap());

        let mut res:Vec<u8> = Vec::new();
        _jolt(0,12, line.as_bytes().to_vec(), &mut res);
        part2 += String::from_utf8(res.clone()).unwrap().parse::<u64>().unwrap();
        //println!("{} -> {}", line, String::from_utf8(res).unwrap());
    }
    (part1, part2)
}

fn process64(inp:&str) -> (u64,u64)
{
    let mut part1 = 0;
    let mut part2 = 0;
    for line in inp.lines() {
        let lineb = line.as_bytes();
//        let res = recursive_joltu64(0,2, line.as_bytes().to_vec(), 0);
        let res = joltu64(2, lineb);
//        let res = jolt2(2, lineb);
        part1 += res;
        // print!("{} -> {}", line, res);

        let res = joltu64(12, lineb);
//        let res = jolt2(12, lineb);
        part2 += res;
        // println!("  {}", res);
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

    let bench_result = run_benchmark(1000, |_| { process64(&input); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process64(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}