// Fastest run (Surface):
//              Acer:     1.484 us
use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;
//use std::iter;

fn process(inp:&str) -> (usize, usize)
{
    let bytes = inp.as_bytes();
    let mut i = 0;
    let mut x:i32 = 0;
    let mut y:i32 = 0;
    let mut d:usize = 0;
    const DX:[i32;4] = [1,0,-1,0];
    const DY:[i32;4] = [0,1,0,-1];
    // let mut _line = 1;
    let mut x2 = 0;
    let mut y2 = 0;
    let mut wx = 10;
    let mut wy = -1;
    while i < bytes.len() {
        let dir = bytes[i]; i+= 1;
        // print!("{}", dir as char);
        // print!("{}",bytes[i] as char);
        let mut n:i32 = (bytes[i] - b'0') as i32; i += 1;
        while bytes[i] != b'\n' {
            // print!("{}",bytes[i] as char);
            n = n*10 + (bytes[i] - b'0') as i32; i += 1;
        }
        // println!("{}{n}", dir as char);
        i += 1;  // skip the newline
        match dir {
            b'N' => {y -= n; wy -= n;},
            b'S' => {y += n; wy += n;},
            b'E' => {x += n; wx += n;},
            b'W' => {x -= n; wx -= n;},
            b'L' => {
                let mut ddir = n/90; 
                d = (d + 4 - ddir as usize) & 3;
                while ddir > 0 {
                    (wx,wy) = (wy,-wx);
                    ddir -= 1;
                }
            },
            b'R' => {
                let mut ddir = n/90; 
                d = (d + ddir as usize) & 3;
                while ddir > 0 {
                    (wx,wy) = (-wy,wx);
                    ddir -= 1;
                }
            },
            b'F' => {
                x += DX[d]*n; 
                y += DY[d]*n;
                x2 += wx*n;
                y2 += wy*n;
            },
            _ => {panic!("Bad direction {d}")},
        }
        // println!("line: {_line} x={x}, y={y}, d={d}, x2={x2}, y2={y2}, wx={wx}, wy={wy}");
        // _line += 1;
    }
    let part1 = (x.abs() + y.abs()) as usize;
    let part2 = (x2.abs() + y2.abs()) as usize;
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
    if input.as_bytes()[input.len()-1] != b'\n' {input.push('\n');}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { _process_1000(&input); }); bench_result.print_stats();

    //process(&input);
    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}