// Fastest run 2844 ns HP

use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;

pub fn process(inp:&str) -> u32
{
    let mut part1 = 0;
    let input = inp.as_bytes();
    let mut i = 0;
    while input[i+2] != b'x' { i += 1; } // skip header block with the shapes, all fit in 3x3

    while i+24 < input.len() {
//        println!("Line: {}", std::str::from_utf8(&input[i..i+24]).unwrap());
        let x = (input[i] - b'0') * 10 + (input[i+1] - b'0');
        let y = (input[i+3] - b'0') * 10 + (input[i+4] - b'0');
        let size = x as u32 * y as u32;
//        println!("x = {}, y = {}, size = {}", x, y, size);
//        assert!(size == 41 * 48);
        let tens = input[i+7] - b'0' + input[i+10] - b'0' + input[i+13] - b'0' + input[i+16] - b'0' + input[i+19] - b'0' + input[i+22] - b'0';
        let ones = input[i+8] - b'0' + input[i+11] - b'0' + input[i+14] - b'0' + input[i+17] - b'0' + input[i+20] - b'0' + input[i+23] - b'0';
        let totalblocks = tens as u32 * 10 + ones as u32;
        if totalblocks*9 <= size { part1 += 1; }
        i += 25; // skip to next line
    }
    part1
}

fn p1k(inp:&str) -> u32
{
    for _ in 0..1000 {
        process(inp);
    }
    0
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { p1k(&input); }); bench_result.print_stats();

    devtime.start();
    let part1 = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}