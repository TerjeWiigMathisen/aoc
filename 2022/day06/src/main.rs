// Fastest run: Surface Pro 8, 5.2 us

//use devtimer::DevTime;
use devtimer::run_benchmark;

fn process(inp:&String, len:usize)->usize
{
    let input = inp.as_bytes();
    let mut lastseen:[u16;32] = [0;32];
    let mut last_duplicate = 0;
    for i in 1.. {  // The input is good, so we can just loop until we find the answer
        let b = (input[i-1] & 31) as usize;
        let ls = lastseen[b] as usize;
        lastseen[b] = i as u16;
        if ls > last_duplicate {
            last_duplicate = ls;
        }
        if i-last_duplicate >= len {
            return i;
        }
    }
    return 0;
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let bench_result = run_benchmark(1000, |_| {
        process(&input,4);
        process(&input,14);
    });
    bench_result.print_stats();

    let part1 = process(&input, 4);
    let part2 = process(&input, 14);
    println!("part1={part1}");
    println!("part2={part2}");
}
