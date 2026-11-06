// Fastest run 248.1 us Acer
// 16.3 us HP

use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;

fn ways(line:usize, pos:usize, lines:&Vec<&str>, cache:&mut Vec<u64>) -> u64
{
    if line+line >= lines.len() { return 1;}
    let key = line*150+pos;
    if cache[key] != 0 { return cache[key]; }

    let bytes = lines[line+line].as_bytes();
    if bytes[pos] == b'^' {
        let w = ways(line+1,pos-1, lines, cache) + ways(line+1,pos+1, lines, cache);
        cache[key] = w;
        return w;
    }
    let w = ways(line+1,pos, lines, cache);
    cache[key] = w;
    w
}

fn process(inp:&str) -> (u64, u64)
{
    let lines = inp.lines().collect::<Vec<&str>>();
    let start = lines[0].find('S').unwrap();
    let mut tach:[u8;160] = [0;160];
    let mut part1 = 0;
    tach[start] = 1;
    let mut left = start;
    let mut right = start;
    let mut ntach:[u8;160] = [0;160];
    let mut l = 2;
    while l < lines.len() {
        let line = lines[l];
        let bytes = line.as_bytes();
        for i in left..=right {
            let tk = tach[i];
            if tk == 0 { continue; }

//            println!("line {} pos {} tk {}", line, i, tk);
            if bytes[i] == b'^' {
                let (p,n) = (i-1,i+1);
                ntach[p] = 1;
                ntach[n] = 1;
                part1 += 1;
                if p < left { left = p; }
                if n > right { right = n; }
            }
            else {
                ntach[i] = 1;
            }
        }
        // println!("line {} left {} right {} splits {}", line, left, right, splits);
        tach = ntach;
        ntach = [0;160];
        l += 2;
    }
    let mut cache:Vec<u64> = vec![0;150*75];
    let part2 = ways(1, start, &lines, &mut cache);
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