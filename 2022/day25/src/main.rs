//aoc 2022 day25
// Fastest run: Surface Pro 8 1456 ns
//                       Acer  651 ns

//use devtimer::DevTime;
use devtimer::run_benchmark;
//use std::arch::x86_64::*;

const BASE:i64 = 5;
const VALUE:[i8;256] = [
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,-1,0,0,0,1,2,0,0,0,0,0,0,0,0,0,0,-2,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];

const DIGITS:[u8;5] = [b'0',b'1',b'2',b'=',b'-'];

fn _frombase(digits:&[u8]) -> i64
{
	let mut n:i64 = 0;
	for i in 0..digits.len() {
		n = n*BASE + (VALUE[digits[i] as usize] as i64);
	}
	n
}

fn tobase(n:i64) -> String
{
	let mut n = n;
    let mut buf:[u8;32] = [0;32];
    let mut p = buf.len();
	loop {
		let r = n % BASE; // Always positive remainder for positive $BASE
		let d = DIGITS[r as usize];
		n = (n-VALUE[d as usize] as i64) / BASE; // Always exact division!
        p -= 1;
        buf[p] = d;
        if n == 0 {break;}
	}
	std::str::from_utf8(&buf[p..buf.len()]).unwrap().to_string()
}

#[inline(never)]
pub fn process(inp:&[u8]) -> (i64,String)
{
    let mut part1 = 0;
    let mut n = 0;
    for i in 0..inp.len() {
        if inp[i] == b'\n' {
            part1 += n;
            n = 0;
            continue;
        }
		n = n*BASE + (VALUE[inp[i] as usize] as i64);
    }
    part1 += n;
    (part1,tobase(part1))
}

pub fn p1k(inp:&[u8]) -> (i64,String)
{
    let mut p1 = 0;
    let mut p2 = "".to_string();
    for _ in 0..1000 {
        (p1,p2) = process(inp);
    }
    (p1,p2)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    // if input.as_bytes()[input.len() - 1] != b'\n' {
    //     input.push('\n');
    // }

    let bench_result = run_benchmark(1000, |_| {
        p1k(&input.as_bytes());
    });
    bench_result.print_stats();

    let display = process(&input.as_bytes());
    println!("part1={}", display.0);
    println!("part2={}", display.1);
//    display.2.show();
}
