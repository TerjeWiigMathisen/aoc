// Fastest run Surface:
//              Acer:  
use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;
use rustc_hash::FxHashMap;

//#[cfg(target_arch = "x86_64")]
//use core::arch::x86_64::_pdep_u64;

fn process(inp:&str) -> (u64, u64)
{
    let bytes = inp.as_bytes();
    let mut i = 0;
    let mut part1 = 0;
    let mut part2 = 0;
    while i < bytes.len() {
    }
    (part1,part2)
}

fn _process_1000(inp:&str) -> (u64, u64)
{
    for _ in 0..1000 {
        process(&inp);
    }
    (0,0)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
//    let fname = "test.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error reading input file");
    if input.as_bytes()[input.len()-1] != b'\n' {input.push('\n');}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { process(&input); }); bench_result.print_stats();

    //process(&input);
    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}