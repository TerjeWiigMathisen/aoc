// day11
// Surface:   29.0 us
// Acer:      12.4 us
use std::fs;
use devtimer::run_benchmark;

fn parse(inp:&[u8], xs:&mut Vec<u8>, ys:&mut Vec<u8>)
{
    let mut i = 0;
    while inp[i] != b'\n' { i += 1;}
    let xlen = i;
    let ylen = (inp.len() + 1) / (i+1);
    i = 0;
    for y in 0..ylen {
        for x in 0..xlen {
            if inp[i] == b'#' {
                xs.push(x as u8);
                ys.push(y as u8);
            }
            i += 1;
        }
        i +=1;
    }
    xs.sort_unstable();
}

fn sum_dist(xs:&[u8]) -> (usize, usize)
{
    let len = xs.len();
    let mut part1 = 0;
    let mut part2 = 0;
    let mut prev = xs[0] as usize;
    for i in 1..len {
        let curr = xs[i] as usize;
        let delta = curr - prev;
        if delta > 0 {
            let gap = delta - 1;
            let dist1 = delta + gap;
            let dist2 = delta + gap * 999999;
            let count = i * (len-i);
            part1 += dist1 * count;
            part2 += dist2 * count;
        }
        prev = curr;
    }
    (part1, part2)
}

fn process(inp:&str)->(usize,usize)
{
    let input = inp.as_bytes();
    let mut xs:Vec<u8> = Vec::new();
    let mut ys:Vec<u8> = Vec::new();
    parse(input, &mut xs, &mut ys);
    let (x1, x2) = sum_dist(&xs);
    let (y1, y2) = sum_dist(&ys);
    (x1+y1, x2+y2)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let input = fs::read_to_string(fname).expect("Error readin input file");

    let bench_result = run_benchmark(1000, |_| {
        process(&input);
    });
    bench_result.print_stats();

    let (part1, part2) = process(&input);
    println!("part1={part1}\npart2={part2}");
}
