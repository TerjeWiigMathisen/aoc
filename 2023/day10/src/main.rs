// day09
// Surface:
// Acer:   
use std::fs;
use devtimer::run_benchmark;

pub fn process(inp:&String)->(i64, i64)
{
    let input = inp.as_bytes();
    let mut part1 = 0;
    let mut part2 = 0;
    (part1, part2)
}

fn _p1k(input:&String)->usize
{
    for _ in 0..1000 {
        process(input);
    }
    return 0;
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
//    let fname = "test.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error reading input file");
    if input.as_bytes()[input.as_bytes().len()-1] != b'\n' as u8 {input.push('\n');}

    let bench_result = run_benchmark(1000, |_| {
        process(&input);
    });
    bench_result.print_stats();

    let res = process(&input);
    println!("part1={}\npart2={}", res.0, res.1);
}
