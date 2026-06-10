// Fastest run (Surface): 1198 ns
//              Acer:      673 ns
use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;

fn parse(inp:&str) -> Vec<u8>
{
    let mut num:Vec<u8> = Vec::with_capacity(1000);
    let mut i = 0;
    let bytes = inp.as_bytes();
    while i < bytes.len() {
        let mut n = (bytes[i]-b'0') as u8; i += 1;
        while bytes[i] >= b'0' {
            n = n*10 + (bytes[i]-b'0') as u8;
            i += 1;
        }
        num.push(n);
        i += 1;
    }
    num
}

fn process(inp:&str) -> (usize, usize)
{
    let mut numbers:Vec<u8> = parse(&inp);
    numbers.push(0);
    numbers.sort_unstable();

    let mut diffs: usize = 0;
    let mut prev = 0;
    for i in 1..numbers.len() {
        let curr = numbers[i];
        let diff = curr - prev - 1;
        diffs += (1 as usize) << (diff*8);
        prev = curr;
    }
    let part1 = (diffs & 255) as usize * ((diffs >> 16) + 1) as usize;

    let mut perm:Vec<usize> = vec![0; numbers.len()];
    perm[0] = 1;
    let mut left = 0;
    for i in 1..numbers.len() {
        while numbers[i] - numbers[left] > 3 { left += 1; }
        let mut sum = perm[left];
        for j in (left+1)..i {
            sum += perm[j];
        }
        perm[i] = sum;
    }
    let part2 = perm[numbers.len()-1];
    (part1,part2)
}

fn _process_1000(inp:&str) -> (usize, usize)
{
    for _ in 0..1000 {
        process(&inp);
    }
    (0,0)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.len()-1] != b'\n' {input.push('\n');}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { _process_1000(&input); }); bench_result.print_stats();

    //process(&input);
    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}