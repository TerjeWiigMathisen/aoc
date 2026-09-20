// day20
// Surface:
// Acer:   

use std::fs;
use devtimer::run_benchmark;

fn process(inp:&str) -> (usize, usize)
{
    let lines:Vec<&[u8]> = inp.lines().map(|s| s.as_bytes()).collect();
    let part1 = 0;

    let part2 = 0;

    (part1, part2)
}

fn main() {
//    let fname = "test.txt"; // instead of args[1]
    let fname = "input.txt"; // instead of args[1]
    let mut input:String = fs::read_to_string(fname).expect("Error reading input file");
    if input.ends_with('\n') { input.pop(); }
    
    let bench_result = run_benchmark(1000, |_| {
        process(&input);
    });
    bench_result.print_stats();

    let res = process(&input);
    println!("part1={}\npart2={}", res.0, res.1);
}
