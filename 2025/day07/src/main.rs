// Fastest run 248.1 us Acer
// 14.7 us HP

use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;

fn ways(line:usize, pos:usize, lines:&Vec<&str>, cache:&mut Vec<u64>, part1: &mut u64) -> u64
{
    if line+line >= lines.len() { return 1;}
    let key = line*150+pos;
    if cache[key] != 0 { return cache[key]; }

    let bytes = lines[line+line].as_bytes();
    if bytes[pos] == b'^' {
        let w = ways(line+1,pos-1, lines, cache, part1) + ways(line+1,pos+1, lines, cache, part1);
        cache[key] = w;
        *part1 += 1;
        return w;
    }
    let w = ways(line+1,pos, lines, cache, part1);
    cache[key] = w;
    w
}

fn process(inp:&str) -> (u64, u64)
{
    let lines = inp.lines().collect::<Vec<&str>>();
    let start = lines[0].find('S').unwrap();
    let mut part1 = 0;
    let mut cache:Vec<u64> = vec![0;150*75];
    let part2 = ways(1, start, &lines, &mut cache, &mut part1);
    (part1, part2)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut fname = "input.txt".to_string();
    if args.len() > 1 { fname = args[1].clone(); }

    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10000, |_| { process(&input); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}