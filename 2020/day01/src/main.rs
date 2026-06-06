// Fastest run (Acer): 4.0 us       (maneatingape: 11)
use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;

fn process(inp:&str) -> (usize, usize)
{
    let mut part1 = 0;
    let mut part2 = 0;
    let bytes = inp.as_bytes();
    let mut numbers:[u16;2020] = [0;2020];
    let mut i = 0;
    let mut seen:Vec<u16> = Vec::new();
    while i < bytes.len() {
        let mut fra = (bytes[i]-b'0') as usize;
        i += 1;
        while bytes[i] >= b'0' {
            fra = fra*10 + (bytes[i] - b'0') as usize;
            i += 1;
        }
        i += 1; // newline
        if fra >= 2020 {continue;}
        seen.push(fra as u16);
        numbers[fra] = 1;
        let diff = 2020-fra;
        if numbers[diff] > 0 {
            part1 = fra * diff;
        }
    }
    for f in 0..seen.len()-2 {
        let first = seen[f] as usize;
        let rem = 2020-first;
        for s in f+1..seen.len()-1 {
            let second = seen[s] as usize;
            if rem > second {
                let diff = rem-second;
                if numbers[diff] > 0 && diff != first && diff != second {
                    part2 = first * second * diff;
                    return (part1, part2)
                }
            }
        }
    }
    (part1, part2)
}

fn main() {
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
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}