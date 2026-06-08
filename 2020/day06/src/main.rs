// Fastest run (Surface): 15.5 us
//              Acer: 5.0 us        vs 38 us
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
    let mut curr:u32 = 0;
    let len = bytes.len();
    loop {
        let b = bytes[i]; i += 1;
        loop {
            let bit = 1 << (b & 31);
            curr |= bit;
            let b = bytes[i]; i += 1;
            if b == b'\n' { break; }
        }
        all &= curr;
        any |= curr;
        curr = 0;
        if bytes[i] == b'\n' { // Is this a double newline ending a section? Safe look-ahead due to input padding!
            part1 += any.count_ones() as usize;
            part2 += all.count_ones() as usize;
            all = u32::MAX;
            any = 0;
            i += 1;
            if i >= len {break} // The input can only end here, so the main loop does not have to test
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
    input.push('\n'); // add extra blank line to simplify code

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