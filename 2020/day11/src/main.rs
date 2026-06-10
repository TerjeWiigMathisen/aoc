// Fastest run (Surface):
//              Acer:    12.3 ms
use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;
//use std::iter;

#[derive(Clone, Debug)]
struct Grid {
    height:usize,
    width:usize,
    neighbors:[i64;8],
    bytes:Vec<u8>,
}

const BORDER:u8 = 0;
const EMPTY:u8 = 1;
const CHAIR:u8 = 2;
const TAKEN:u8 = 3;

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
    let height = bytes.len() / stride;
    let s64 = stride as i64;
    let mut g = Grid{height:height,width:width,
        neighbors:[-1-s64, -s64, 1-s64, -1, 1, -1+s64, s64, 1+s64],
        bytes:vec![]};
    let border = b'\n';
    for _ in 0..=stride {g.bytes.push(border)}
    for i in 0..bytes.len() {g.bytes.push(bytes[i])}
    for _ in 0..=stride {g.bytes.push(border)}
    g
}

fn gen1(grid:&Grid) -> (usize, Grid)
{
    let mut cnt = 0;
    let mut newgrid = grid.clone();
    for p in grid.width..(grid.bytes.len()-grid.width) {
        let seat = grid.bytes[p];
        if seat == b'.' || seat <= b' ' {continue}
        let mut occupied = 0;
        for d in grid.neighbors {
            if grid.bytes[(p as i64 + d) as usize] == b'#' { occupied += 1}
        }
        if seat == b'#' {
            if occupied >= 4 {
                newgrid.bytes[p] = b'L';
                cnt += 1;
            }
        }
        else { // seat = b'L'
            if occupied == 0 {
                newgrid.bytes[p] = b'#';
                cnt += 1;
            }
        }
    }
    (cnt, newgrid)
}

fn gen2(grid:&Grid) -> (usize, Grid)
{
    let mut cnt = 0;
    let mut newgrid = grid.clone();
    for p in grid.width..(grid.bytes.len()-grid.width) {
        let seat = grid.bytes[p];
        if seat == b'.' || seat == b'\n' {continue}
        let mut occupied = 0;
        for d in grid.neighbors {
            let mut pd = p;
            loop {
                pd = (pd as i64 + d) as usize;
                match grid.bytes[pd as usize] {
                    b'\n' => {break},
                    b'.' => {},
                    b'#' => {occupied += 1; break},
                    b'L' => {break},
                    _ => {panic!("Bad letter in grid!")}
                }
            }
        }
        if seat == b'#' {
            if occupied >= 5 {
                newgrid.bytes[p] = b'L';
                cnt += 1;
            }
        }
        else { // seat = b'L'
            if occupied == 0 {
                newgrid.bytes[p] = b'#';
                cnt += 1;
            }
        }
    }
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
        part1 += (seat == b'#') as usize;
    }

    grid = grid2.clone();
    for _gen in 0.. {
        (cnt, grid) = gen2(&grid);
        if cnt == 0 {break}
    }
    let mut part2 = 0;
    for p in grid.width..(grid.bytes.len()-grid.width) {
        let seat = grid.bytes[p];
        part2 += (seat == b'#') as usize;
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
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.len()-1] != b'\n' {input.push('\n');}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { process(&input); }); bench_result.print_stats();

    //process(&input);
    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}