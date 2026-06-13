// Fastest run (Surface):
//              Acer:     1.077 us
use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;

fn process(inp:&str) -> (usize, usize)
{
    let bytes = inp.as_bytes();
    let mut i = 0;
    let mut buses:Vec<usize> = Vec::new();

    let mut start:usize = 0;
    while bytes[i] != b'\n' {
        start = start*10 + (bytes[i] - b'0') as usize;
        i += 1;
    }
    let mut earliest = usize::MAX;
    let mut part1 = 0;
    while i < bytes.len() {
        let b = bytes[i]; i += 1;
        if b == b'x' {
            buses.push(usize::MAX);
        }
        else {
            let mut n = (b - b'0') as usize;
            while bytes[i] != b',' {
                n = n*10 + (bytes[i] - b'0') as usize; i += 1;
            }
            buses.push(n);
            let loops = start/n;
            let next = (loops+1)*n;
            let wait = next-start;
            if wait < earliest {
                earliest = wait;
                part1 = wait * n;
            }
        }
        i += 1;
    }

    let mut prod = buses[0];
    let mut part2 = prod;
    for idx in 1..buses.len() {
        let id = buses[idx];
        if id == usize::MAX {continue}
        //println!("product = {prod}");
        while ((part2 + idx) % id) != 0 { 
            part2 += prod;
        }
        prod *= id;
    }
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
//    let fname = "test.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error reading input file");
    if input.as_bytes()[input.len()-1] == b'\n' {input.pop();}
    if input.as_bytes()[input.len()-1] != b',' {input.push(',');}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { _process_1000(&input); }); bench_result.print_stats();

    //process(&input);
    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}