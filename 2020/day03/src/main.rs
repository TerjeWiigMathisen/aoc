// Fastest run (Acer): 1205 ns  vs ref: 8 us
use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;

fn count_collisions(bytes:&[u8], width:usize, end:usize, dx:usize, dy:usize) -> usize
{
    let mut x = 0;
    //let mut y = 0;
    let mut pos = 0;
    let mut hits = 0;
    let dystep = dy*(width+1);
    while pos < end {
        hits += (bytes[pos] == b'#') as usize;
        x += dx; pos += dx;
        if x >= width {
            x -= width;
            pos -= width;
        }
        pos += dystep;
    }
    hits
}

fn process(inp:&str) -> (usize, usize)
{
    let bytes = inp.as_bytes();
    let mut width = 0;
    while bytes[width] != b'\n' {
        width += 1;
    }
    let end = bytes.len();
    let part1 = count_collisions(bytes, width, end, 3, 1);
    let mut part2 = part1;
    for (dx, dy) in [(1,1),(5,1),(7,1),(1,2)] {
        part2 *= count_collisions(bytes, width, end, dx, dy);
    }
    (part1, part2)
}

fn process_1000(inp:&str) ->(usize,usize)
{
    let (mut part1, mut part2) = (0,0);
    for _ in 0..1000 {
        let (p1, p2) = process(&inp);
        (part1,part2) = (p1,p2);
    }
    (part1,part2)
}

fn main() {
//    let bench_result = run_benchmark(10, |_| {bench_permute();}); bench_result.print_stats();

    //panic!("Stop now");

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.len()-1] != b'\n' {input.push('\n');}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process_1000(&input); }); bench_result.print_stats();

    //process(&input);
    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}