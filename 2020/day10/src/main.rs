// Fastest run (Surface): 14.7 us
//              Acer:      7.2 us
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
    let mut numbers:Vec<u8> = parse(&inp); //inp.lines().map(|x| x.parse::<u8>().unwrap()).collect();
    numbers.push(0);
    numbers.sort_unstable();
    //println!("Numbers: {numbers:?}");
    let mut diffs = [0; 3];
    for i in 1..numbers.len() {
        let diff = numbers[i]-numbers[i-1];
        //if diff <= 3 && diff >= 1 {
            diffs[diff as usize - 1] += 1;
        //}
    }
    //println!("Diffs: {diffs:?}");
    let part1 = diffs[0] as usize * (diffs[2]+1) as usize;
    let mut part2 = 0;
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
    println!("Total time {} us for 1000 runs",devtime.time_in_micros().unwrap());
}