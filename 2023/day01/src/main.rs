// day1
// Surface:  75.9 us
// Acer:

use std::fs;
use devtimer::run_benchmark;

pub fn process(inp:&String, ilen:usize)->(u32, u32)
{
    let input = inp.as_bytes();

    let digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut third:[u16;256] = [0;256];
    for j in 0..9 {
        third[digits[j].as_bytes()[2] as usize] |= 1 << j;
        third[digits[j].as_bytes()[0] as usize] |= 1 << 15;
    }

    let mut part1 = 0;
    let mut part2= 0;
    let mut first = 0;
    let mut first2 = 0;
    let mut pair = 0;
    let mut pair2 = 0;
    let mut i = 0;
    while i < ilen {
        let b = input[i]; i += 1;
        if b == b'\n' {
            part1 += pair;
            first = 0;
            part2 += pair2;
            first2 = 0;
            continue;
        }
        if b >= b'0' && b <= b'9' {
            let digit = (b - b'0') as u32; 
            if first == 0 {
                first = digit * 10;
            }
            if first2 == 0 {
                first2 = digit * 10;
            }
            pair = first + digit;
            pair2 = first2 + digit;
            continue;
        }
        let f = third[b as usize];
        if f < (1<<15) {continue;}
        i -= 1;
        let mut t = third[input[i+2] as usize] & 511;
        while t != 0 {
            let j = t.trailing_zeros() as usize;
            if input[i..i+digits[j].len()] == *digits[j].as_bytes() {
                let digit = (j+1) as u32;
                if first2 == 0 {
                    first2 = digit * 10;
                }
                pair2 = first2 + digit;
                i += digits[j].len() - 1;
                break;
            }
            t ^= 1 << j;
        }
        i += 1;
    }
    return (part1, part2);
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] != b'\n' as u8 {input.push('\n');}
    let ilen = input.len();
    for _ in 0..5 {input.push(' ');}

    let bench_result = run_benchmark(1000, |_| {
        process(&input, ilen);
    });
    bench_result.print_stats();

    let part1 = process(&input, ilen);
    println!("part1={}\npart2={}", part1.0, part1.1);
}
