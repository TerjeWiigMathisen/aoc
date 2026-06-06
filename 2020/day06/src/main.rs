// Fastest run (Surface): 15.5 us
//              Acer: 4.4 us        vs 38 us
use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;

fn process(inp:&str) -> (usize, usize)
{
    let bytes = inp.as_bytes();
    let mut i = 0;
    let mut part1 = 0;
    let mut part2 = 0;
    let mut all:u32 = u32::MAX;
    let mut any:u32 = 0;
    let mut curr = 0;
    let len = bytes.len();
    while i < len {
        let b = bytes[i]; i += 1;
        if b == b'\n' {
            all &= curr;
            any |= curr;
            curr = 0;
            if (i >= len) || (bytes[i] == b'\n') {
                i += 1;
                part1 += any.count_ones() as usize;
                part2 += all.count_ones() as usize;
                all = u32::MAX;
                any = 0;
            }

        } else { //if b >= b'a' {
            let bit = 1 << (b & 31);
            curr |= bit;
        }
    }
    (part1, part2)
}

fn process_1000(inp:&str) ->(usize,usize)
{
    let (mut part1, mut part2) = (0,0);
    for _ in 0..1000 {
        let (p1, p2) = process(&inp);
        part1 += p1; part2 += p2;
    }
    (part1,part2)
}

fn main() {
//    let bench_result = run_benchmark(10, |_| {bench_permute();}); bench_result.print_stats();

    //panic!("Stop now");

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.len()-1] != b'\n' {input.push('\n');}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process(&input); }); bench_result.print_stats();

    //process(&input);
    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us for 1000 runs",devtime.time_in_micros().unwrap());
}