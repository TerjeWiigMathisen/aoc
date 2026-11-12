// Fastest run 2844 ns HP
//             2465 ns Acer
// HP with slice 71 ns, 2.2 us single run
// Acer 1.7 us single run

use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;

#[inline(never)]
pub fn process(inp:&str) -> u32
{
    let mut part1 = 0;
    let inpu = inp.as_bytes();
    let mut i = 0;
    while inpu[i+2] != b'x' { i += 1; } // skip header block with the shapes, all fit in 3x3

    while i+24 < inpu.len() {
        let input:[u8;24] = inpu[i..i+24].try_into().unwrap();
//        println!("Line: {}", std::str::from_utf8(&input[i..i+24]).unwrap());
        let x = (input[0] - b'0') * 10 + (input[1] - b'0');
        let y = (input[3] - b'0') * 10 + (input[4] - b'0');
        let size = x as u32 * y as u32;
        let tens = input[7] - b'0' + input[10] - b'0' + input[13] - b'0' + input[16] - b'0' + input[19] - b'0' + input[22] - b'0';
        let ones = input[8] - b'0' + input[11] - b'0' + input[14] - b'0' + input[17] - b'0' + input[20] - b'0' + input[23] - b'0';
        let totalblocks = tens as u32 * 10 + ones as u32;
        if totalblocks*9 <= size { part1 += 1; }
        i += 25; // skip to next line
    }
    part1
}

pub fn p1k(inp:&str) -> u32
{
    let mut part1 = 0;
    for _ in 0..1000 {
        part1 += process(inp);
    }
    part1
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