// Fastest run (Surface):
//              Acer:    11.495 ms
use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;
//use std::iter;

#[derive(Clone, Debug)]
struct Grid {
    stride:usize,
    width:usize,
    start:usize,
    bytes:Vec<u8>,
}

const BORDER:u8 = 0;
const EMPTY:u8 = 1;
const CHAIR:u8 = 2;
const TAKEN:u8 = 4;

fn parse(inp:&str) -> Grid
{
    let bytes = inp.as_bytes();
    let mut width = 0;
    for i in 0..bytes.len() {
        if bytes[i] == b'\n' {
            width = i;
            break;
        }
    }
    let stride = width+1;
    let s64 = stride as i64;
    let mut g = Grid{stride:stride,width:width,start:stride+1,bytes:vec![]};
    let border = BORDER;
    for _ in 0..=stride {g.bytes.push(border)}
    for i in 0..bytes.len() {
        let cell = match bytes[i] {
            b'\n' => BORDER,
            b'.' => EMPTY,
            b'L' => CHAIR,
            b'#' => TAKEN,
            _ => BORDER,
        };
        g.bytes.push(cell);
    }
    for _ in 0..=stride {g.bytes.push(border)}
    //println!("{:?}", g);
    g
}

fn gen1(grid:&Grid) -> (usize, Grid)
{
    let mut cnt = 0;
    let mut newgrid = grid.clone();
    for p in grid.start..(grid.bytes.len()-grid.stride) {
        let seat = grid.bytes[p];
        if seat <= EMPTY {continue}
        let above = p - grid.stride;
        let below = p + grid.stride;
        let mut occupied = (grid.bytes[above-1] & TAKEN) + (grid.bytes[above] & TAKEN) + (grid.bytes[above+1] & TAKEN);
        occupied += (grid.bytes[p-1] & TAKEN) + (grid.bytes[p+1] & TAKEN);
        occupied += (grid.bytes[below-1] & TAKEN) + (grid.bytes[below] & TAKEN) + (grid.bytes[below+1] & TAKEN);
        //println!("seat {p} = {seat}, {occupied} neighbors");
        if seat == TAKEN {
            if occupied >= 4*TAKEN {
                newgrid.bytes[p] = CHAIR;
                cnt += 1;
            }
        }
        else { // seat = b'L'
            if occupied == 0 {
                newgrid.bytes[p] = TAKEN;
                cnt += 1;
            }
        }
    }
    println!("part1 {cnt} modified chairs");
    (cnt, newgrid)
}

fn gen2(grid:&Grid) -> (usize, Grid)
{
    let mut cnt = 0;
    let mut newgrid = grid.clone();
    for p in grid.start..(grid.bytes.len()-grid.stride) {
        let seat = grid.bytes[p];
        if seat <= EMPTY {continue}

        let above = p - grid.stride;
        let below = p + grid.stride;
        let mut occupied = 0;
        let stride = grid.stride as i64;
        for delta in [-stride-1,-stride,-stride+1,-1,1,stride-1,stride,stride+1] {
            let mut dp = p;
            loop {
                dp = (dp as i64 + delta) as usize;
                match grid.bytes[dp] {
                    BORDER => {break},
                    EMPTY => {},
                    CHAIR => {break},
                    TAKEN => {occupied += 1; break}
                    _ => {},
                }
            }
        }
        if seat == TAKEN {
            if occupied >= 5 {
                newgrid.bytes[p] = CHAIR;
                cnt += 1;
            }
        }
        else { // seat = b'L'
            if occupied == 0 {
                newgrid.bytes[p] = TAKEN;
                cnt += 1;
            }
        }
    }
    println!("part2 {cnt} modified chairs");
    (cnt, newgrid)
}

fn process(inp:&str) -> (usize, usize)
{
    let mut grid = parse(&inp);
    let grid2 = grid.clone();
    //println!("{:?}", grid);
    let mut cnt;
    for _gen in 0.. {
        (cnt, grid) = gen1(&grid);
        if cnt == 0 {break}
    }
    let mut part1 = 0;
    for p in grid.width..(grid.bytes.len()-grid.width) {
        let seat = grid.bytes[p];
        part1 += (seat == TAKEN) as usize;
    }

    grid = grid2.clone();
    for _gen in 0.. {
        (cnt, grid) = gen2(&grid);
        if cnt == 0 {break}
    }
    let mut part2 = 0;
    for p in grid.width..(grid.bytes.len()-grid.width) {
        let seat = grid.bytes[p];
        part2 += (seat == TAKEN) as usize;
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
    let mut input = fs::read_to_string(fname).expect("Error reading input file");
    if input.as_bytes()[input.len()-1] != b'\n' {input.push('\n');}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1, |_| { process(&input); }); bench_result.print_stats();

    //process(&input);
    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}