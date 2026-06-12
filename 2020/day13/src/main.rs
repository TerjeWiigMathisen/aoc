// Fastest run (Surface):
//              Acer:     1.218 us
use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;
//use std::iter;

struct Bus {
    id:u32,
}

fn process(inp:&str) -> (u32, u32)
{
    let bytes = inp.as_bytes();
    let mut i = 0;
    let mut buses:Vec<Bus> = Vec::new();

    let mut start:u32 = 0;
    while bytes[i] != b'\n' {
        start = start*10 + (bytes[i] - b'0') as u32;
        i += 1;
    }
    //print!("start={start}");
    i += 1;
    let mut earliest = u32::MAX;
    let mut part1 = 0;
    while i < bytes.len() {
        let b = bytes[i]; i += 1;
        if b == b'x' {
            buses.push(Bus{id:u32::MAX});
        }
        else {
            let mut n = (b - b'0') as u32;
            while bytes[i] != b',' {
                n = n*10 + (bytes[i] - b'0') as u32; i += 1;
            }
            buses.push(Bus{id:n});
            //print!(",{n}");
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
    //println!();

    let mut prod = buses[0].id;
    let mut part2 = prod;
    //println!("Bus {prod} leaves at {part2}+0");
    for idx in 1..buses.len() {
        let id = buses[idx].id;
        if id == u32::MAX {continue}
        //println!("product = {prod}");
        while ((part2 + idx as u32) % id) != 0 { 
            part2 += prod;
            //println!("part2 = {part2}");
        }
        prod *= id;
        //println!("Bus {id} leaves at {part2}+{idx}");
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