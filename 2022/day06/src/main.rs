// Fastest run: Surface Pro 8, 5.2 us
//              Acer 2.564 us
//              HP 2.504 us

//use devtimer::DevTime;
use devtimer::run_benchmark;

fn process(inp:&String)-> (usize, usize)
{
    let input = inp.as_bytes();
    let mut lastseen:[u16;32] = [0;32];
    let mut last_duplicate = 0;
    let p1;
    let mut i = 0;
    loop {  // The input is good, so we can just loop until we find the answer
        i += 1;
        let b = (input[i-1] & 31) as usize;
        let ls = lastseen[b] as usize;
        lastseen[b] = i as u16;
        if ls > last_duplicate {
            last_duplicate = ls;
        }
        if i-last_duplicate >= 4 {
            p1 = i;
            break;
        }
    }
    loop {  // The input is good, so we can just loop until we find the answer
        i += 1;
        let b = (input[i-1] & 31) as usize;
        let ls = lastseen[b] as usize;
        lastseen[b] = i as u16;
        if ls > last_duplicate {
            last_duplicate = ls;
        }
        if i-last_duplicate >= 14 {
            return (p1,i);
        }
    }
}

fn process1k(inp:&String)-> (usize, usize)
{
    let mut p1 = 0;
    let mut p2 = 0;
    for _ in 0..1000 {
        let (x, y) = process(inp);
        p1 = x;
        p2 = y;
    }
    (p1, p2)
}


fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let bench_result = run_benchmark(1000, |_| {
        process1k(&input);
    });
    bench_result.print_stats();

    let (part1, part2) = process(&input);
    println!("part1={part1}");
    println!("part2={part2}");
}
