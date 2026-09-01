// day1
// Surface:   46.4 us
// Acer:      59.2 us

use std::fs;
use devtimer::run_benchmark;

pub fn process(inp:&String, ilen:usize)->(u32, u32)
{
    let input = inp.as_bytes();

    let digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut third:[u16;256] = [0;256];
    for j in 0..9 {
        let dig8 = digits[j].as_bytes();
        third[dig8[2] as usize] |= 1 << j;
        third[dig8[0] as usize] |= 1 << 15;
        let last = dig8.len()-1;
        third[dig8[last] as usize] |= 1 << 14;
        third[dig8[last-2] as usize] |= 1 << j;
    }
//    let skip = [1,1,3,3,2,2,3,3,2]; //forward skip after finding text

    let mut part1 = 0;
    let mut part2 = 0;
    let mut first;
    let mut first2 = 0;
    let mut pair;
    let mut pair2 = 0;
    let mut i = 0;
    while i < ilen {
        let _linestart = i;
        loop { // Forward scan
            let b = input[i]; i += 1;
            if /* b >= b'0' && */ b <= b'9' {
                let digit = (b - b'0') as u32; 
                first = digit * 10;
                if first2 == 0 {
                    first2 = digit * 10;
                }
                break;
            }
            if first2 != 0 {continue;}

            let f = third[b as usize];
            if f < (1<<15) {continue;}
            i -= 1;
            let mut t = third[input[i+2] as usize] & 511;
            while t != 0 {
                let j = t.trailing_zeros() as usize;
                if input[i..i+digits[j].len()] == *digits[j].as_bytes() {
                    let digit = (j+1) as u32;
                    first2 = digit * 10;
                    break;
                }
                t ^= 1 << j;
            }
            i += 1;
        }
        while input[i] != b'\n' {i += 1;}

//        println!("{}",String::from_utf8(input[linestart..i].to_vec()).unwrap());
//        println!("first={first}, first2={first2}");
//        assert!(first != 0 && first2 != 0);
        let mut k = i;
        loop {
            k -= 1;
            let b = input[k];
            if b <= b'9' {
                let digit = (b-b'0') as u32;
                pair = first + digit;
                if pair2 == 0 {
                    pair2 = first2 + digit;
                }
                break;
            }
            if pair2 != 0 { continue; }
            let f = third[b as usize];
            if f & (1<<14) == 0 {continue;}
            let mut t = third[input[k-2] as usize] & 511;
            while t != 0 {
                let j = t.trailing_zeros() as usize;
                if input[k-digits[j].len()+1..=k] == *digits[j].as_bytes() {
                    let digit = (j+1) as u32;
                    pair2 = first2 + digit;
                    break;
                }
                t ^= 1 << j;
            }
        }
        i += 1;
        part1 += pair;
        part2 += pair2;
//        println!("pair={pair}, pair2={pair2}")
        //first = 0;
        first2 = 0;
        //pair = 0;
        pair2 = 0;
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
