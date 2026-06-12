// Fastest run (Surface): 6.32 ms
//              Acer:     9.263 ms
use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;
//use std::iter;

const BORDER:u8 = 0;
const EMPTY:u8 = 1;
const CHAIR:u8 = 2;
const TAKEN:u8 = 4;

#[derive(Clone, Debug)]
struct Cell2 {
    seat:u8,
    dirty:bool,
    nbors:i16,
    neighbors:[u16; 8],
}

#[derive(Clone, Debug)]
struct Grid2 {
    stride:usize,
    width:usize,
    start:usize,
    cells:Vec<Cell2>,
}

fn parse2(inp:&str, multi:bool) -> Grid2
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
    let mut g = Grid2{stride:stride,width:width,start:stride+1,cells:vec![]};
    let border = BORDER;
    for _ in 0..=stride {g.cells.push(Cell2{seat:border,dirty:false,nbors:0,neighbors:[0; 8]})}
    for i in 0..bytes.len() {
        let seat = match bytes[i] {
            b'\n' => BORDER,
            b'.' => EMPTY,
            b'L' => CHAIR,
            b'#' => TAKEN,
            _ => BORDER,
        };
        g.cells.push(Cell2{seat:seat,dirty:true,nbors:0,neighbors:[0; 8]});
    }
    for _ in 0..=stride {g.cells.push(Cell2{seat:border,dirty:false,nbors:0,neighbors:[0; 8]})}

    // Initialize neighbor addresses and counts
    let s64 = stride as i64;
    for p in stride+1..(g.cells.len()-stride) {
        if g.cells[p].seat <= EMPTY {continue}
        let mut nb = 0;
        for (idx, delta) in [-s64-1,-s64,-s64+1,-1,1,s64-1,s64,s64+1].iter().enumerate() {
            let mut dp = p as i64;
            loop {
                dp += delta;
                match g.cells[dp as usize].seat {
                    BORDER => {break},
                    EMPTY => {if !multi {break}},
                    CHAIR => {break},
                    TAKEN => {nb += 1; break}
                    _ => {},
                }
            }
            g.cells[p].neighbors[idx] = dp as u16;
        }
        g.cells[p].nbors = nb as i16;
    }
    //println!("{:?}", g);
    g
}

fn gen2nb(grid:&mut Grid2, crowded:i16) -> usize
{
    let mut cnt = 0;
    //let mut newgrid = grid.clone();
    let mut changes:Vec<u16> = Vec::with_capacity(7000);
    for p in grid.start..(grid.cells.len()-grid.stride) {
        let cell = &mut grid.cells[p];
        if cell.seat <= EMPTY || cell.dirty == false {continue}

        cell.dirty = false;
        if cell.seat == TAKEN {
            if cell.nbors >= crowded {
                cell.seat = CHAIR;
                cell.dirty = true;
                cnt += 1;
            }
        }
        else { // seat = b'L'
            if cell.nbors == 0 {
                cell.seat = TAKEN;
                cell.dirty = true;
                cnt += 1;
            }
        }
        if cell.dirty == false {continue}
        changes.push(p as u16);
    }
    for p in changes {
        let delta = if grid.cells[p as usize].seat == TAKEN {1} else {-1};
        for nboridx in 0..8 {
            let nborp = grid.cells[p as usize].neighbors[nboridx] as usize;
            grid.cells[nborp].nbors += delta;
            grid.cells[nborp].dirty = true;
        }
    }
    //println!("part2 {cnt} modified chairs");
    cnt
}

fn taken_seat_count(grid:&Grid2) -> usize
{
    let mut cnt = 0;
    for p in grid.start..(grid.cells.len()-grid.stride) {
        let seat = grid.cells[p].seat;
        cnt += (seat == TAKEN) as usize;
    }
    cnt
}

fn process(inp:&str) -> (usize, usize)
{
    let mut grid = parse2(&inp, false);
    //println!("{:?}", grid);
    let mut cnt;
    for _gen in 0.. {
        cnt = gen2nb(&mut grid, 4);
        if cnt == 0 {break}
        //break;
    }
    let mut part1 = 0;
    for p in grid.start..(grid.cells.len()-grid.stride) {
        let seat = grid.cells[p].seat;
        part1 += (seat == TAKEN) as usize;
    }

    grid = parse2(&inp, true);
    for _gen in 0.. {
        cnt = gen2nb(&mut grid, 5);
        if cnt == 0 {break}
    }
    let mut part2 = 0;
    for p in grid.start..(grid.cells.len()-grid.stride) {
        let seat = grid.cells[p].seat;
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

    let bench_result = run_benchmark(100, |_| { process(&input); }); bench_result.print_stats();

    //process(&input);
    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}