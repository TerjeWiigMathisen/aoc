// Fastest run Surface 522 us
// Dirty list 990 us

// HP 71.7

use std::fs;
use std::collections::VecDeque;
use devtimer::DevTime;
use devtimer::run_benchmark;

fn removed3(start:usize, end:usize, buffer:&mut Vec<u8>, remove:&mut VecDeque<u16>)-> usize
{
    for i in start..end {
        let b = buffer[i];
        if (b & 1) != 0 && b < 8 { 
            remove.push_back(i as u16);
        }
    }
    let remcnt = remove.len();
    remcnt
}

fn removeloop(stride:usize, buffer:&mut Vec<u8>, remove:&mut VecDeque<u16>)-> usize
{
    let mut remcnt = 0;
    while let Some(i) = remove.pop_front() {
        let i:usize = i as usize;
        for k in [i-stride-1, i-stride, i-stride+1, i-1, i+1, i+stride-1, i+stride, i+stride+1] {
            buffer[k] -= 2;
            if buffer[k] == 7 {
                remove.push_back(k as u16);
            }
        }
        buffer[i] -= 1;
        remcnt += 1;
    }
    remcnt
}

fn process(inp:&str) -> (usize, usize)
{
    let linelen = inp.find('\n').unwrap();
    let stride = linelen+1;
    let start = stride+1;
    let bytes = inp.as_bytes();
    let blen = bytes.len();
    let end = start + blen;
    let mut buffer:Vec<u8> = vec![0;stride*2+blen+2];
    let mut i = start;
    for j in 0..blen {
        let b = bytes[j];
        if b == b'@' {
            buffer[i-stride-1] += 2;
            buffer[i-stride] += 2;
            buffer[i-stride+1] += 2;
            buffer[i-1] += 2;
            buffer[i] += 1;
            buffer[i+1] += 2;
            buffer[i+stride-1] += 2;
            buffer[i+stride] += 2;
            buffer[i+stride+1] += 2;
        }
        i += 1;
    }
    let mut remove:VecDeque<u16> = VecDeque::with_capacity(4000);
    let part1 = removed3(start, end, &mut buffer, &mut remove);
    let part2 = removeloop(stride, &mut buffer, &mut remove);
    (part1, part2)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut fname = "input.txt".to_string();
    if args.len() > 1 { fname = args[1].clone(); }

    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process(&input); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}