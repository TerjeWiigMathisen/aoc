// day06
// Surface:   176 ns
// Acer:       92 ns
use std::fs;
use devtimer::run_benchmark;

fn ways_to_win(tid:usize, dist:usize) -> usize
{
	// dist = button_press * (tid-button_press) > distance_to_beat
	// -b^2 + b*t - distance_to_beat = 0
	// b^2 - b*t + distance_to_beat = 0
	// (b - t/2)^2 + distance_to_beat - (t/2)^2
	// b = t/2 +- sqrt((t/2)^2-distance_to_beat)
    let t2 = tid as f64 * 0.5;
    let root = ((t2 * t2) as f64 - (dist as f64)).sqrt();
    let mut min = (t2 - root) as usize;
    while (tid-min) * min <= dist { min += 1; }
    let mut max = (t2 + root) as usize;
    while (tid-max) * max <= dist { max -= 1; }
    return max - min + 1;
}

pub fn process(inp:&String)->(usize, usize)
{
    let input = inp.as_bytes();
    let mut i = 9;
    let mut times:Vec<usize> = Vec::new();
    let mut time2 = 0;
    loop {
        while input[i] == b' ' { i += 1; }
        let mut n = (input[i] - b'0') as usize; i += 1;
        time2 = time2 * 10 + n;
        while input[i] >= b'0' && input[i] <= b'9' {
            let d = (input[i] - b'0') as usize;
            n = n * 10 + d;
            time2 = time2 * 10 + d;
            i += 1;
        }
        times.push(n);
        if input[i] == b'\n' {break;}
        i += 1;
    }
    let mut distances:Vec<usize> = Vec::new();
    let mut distance2 = 0;
    i += 10; // Skip "\nDistance:"
    loop {
        while input[i] < b'0' { i += 1; }
        let mut n = (input[i] - b'0') as usize; i += 1;
        distance2 = distance2 * 10 + n;
        while input[i] >= b'0' && input[i] <= b'9' {
            let d = (input[i] - b'0') as usize;
            n = n * 10 + d;
            distance2 = distance2 * 10 + d;
            i += 1;
        }
        distances.push(n);
        if input[i] == b'\n' {break;}
        i += 1;
    }
    let mut part1 = 1;
    for j in 0..times.len() {
        let w = ways_to_win(times[j], distances[j]);
        part1 *= w;
    }
    let part2 = ways_to_win(time2, distance2);
    return (part1, part2);
}

fn p1k(input:&String)->usize
{
    for _ in 0..1000 {
        process(input);
    }
    return 0;
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
//    let fname = "test.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] != b'\n' as u8 {input.push('\n');}

    let bench_result = run_benchmark(1000, |_| {
        p1k(&input);
    });
    bench_result.print_stats();

    let res = process(&input);
    println!("part1={}\npart2={}", res.0, res.1);
}
