// Fastest run 26.4 us HP (tailscan 41.2)
// Acer 37.4
// Surface 80.3

use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;

fn joltu64(digits:usize, n:&[u8]) -> u64
{
	let mut digits = digits;
    let mut first = 0;
    let mut res = 0;
    let nlen = n.len();
    let mut m = 0;
    let mut ntail:Vec<u8> = vec![0;nlen];
    for i in 1..nlen {
        let idx = nlen-i;
        if n[idx] > m {m = n[idx];}
        ntail[idx] = m;
    }
    loop {
        digits -= 1;
        let mut max:u64 = 0;
        let mut pos:usize = first;
        for p in first..nlen-digits {
            if max >= ntail[p] as u64 { break; }
            let np = n[p] as u64;
            if np > max {
                max = n[p] as u64;
                pos = p;
                if n[p] == ntail[p] {
                    break;
                }
            }
        }
        res = res*10 + max - b'0' as u64;
        // println!("{res}");
        first = pos+1;
        if digits ==  0 {
            break;
        }
    }
    res
}

fn process64(inp:&str) -> (u64,u64)
{
    let mut part1 = 0;
    let mut part2 = 0;
    for line in inp.lines() {
        let lineb = line.as_bytes();
        let res = joltu64(2, lineb);
        part1 += res;

        let res = joltu64(12, lineb);
        part2 += res;
    }
    (part1, part2)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut fname = "input.txt".to_string();
    if args.len() > 1 { fname = args[1].clone(); }

    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10000, |_| { process64(&input); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process64(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}