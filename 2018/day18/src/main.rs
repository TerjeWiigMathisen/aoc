use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

fn disp(grid:&Vec<u8>, width:usize, showgrid:bool)
{
    let mut trees = 0; let mut lumb = 0;
    for y in 1..width-1 {
        for x in 1..width-1 {
            let b = grid[y*width+x];
            if showgrid {print!("{}",b as char);}
            if b == '|' as u8 {trees += 1}
            else if b == '#' as u8 {lumb += 1}
        }
        if showgrid {println!("");}
    }
    println!("trees = {trees}, lumberyards = {lumb} - > {}", trees*lumb);
}

fn process(inp:String, maxgen:usize) -> (i64,i64)
{
    let lines:Vec<String> = inp.split("\n").map(|s| s.to_string()).collect();
    let width = lines[0].len()+2;
    let height = lines.len()+2;
    let _xmin = 1; let _ymin = 1; let xmax = lines[0].len(); let ymax = lines.len();
    let mut grid:Vec<u8> = vec!['.' as u8;width*height];
    let mut y = 1;
    for li in lines {
        let mut pos = y*width+1;
        let bytes = li.as_bytes();
        for i in 0..bytes.len() {
            grid[pos] = bytes[i];
            pos += 1;
        }
        y += 1;
    }
    let mut seen:HashMap<String,usize> = HashMap::new();
    let mut duplicate_found = false;
    let mut generations = 0;
    while generations < maxgen {
        let old = grid.to_owned().to_vec();
        for y in 1..=ymax {
            for x in 1..=xmax {
                let mut trees = 0; let mut lumb = 0;
                for (dx,dy) in [(-1,-1),(0,-1),(1,-1),(-1,0),(1,0),(-1,1),(0,1),(1,1)] {
                    let b = old[((y as i64 + dy)*width as i64 + (x as i64+dx)) as usize];
                    if b == '|' as u8 {trees += 1;}
                    else if b == '#' as u8 {lumb += 1;}
                }
                let c = old[y*width+x];
                if c == '.' as u8 && trees >= 3 {
                    grid[y*width+x] = '|' as u8;
                }
                else if c == '|' as u8 && lumb >= 3 {
                    grid[y*width+x] = '#' as u8;
                }
                else if c == '#' as u8 && trees*lumb == 0 {
                    grid[y*width+x] = '.' as u8;
                }
            }
        }
//        print!("Gen {generations}: "); disp(&grid, width,false);
        if !duplicate_found {
            let gg = grid.to_owned().to_vec();
            let key:String = String::from_utf8(gg).unwrap();
            if seen.contains_key(&key) {
                let prev = seen.get(&key).unwrap();
                let period = generations-prev;
                println!("Period (len = {period}) found: Gen {prev} == gen {generations}");
                let skip = (maxgen - generations) / period * period;
                generations += skip;
                println!("Skipping {skip} generations");
    //            panic!("Wait here");
                duplicate_found = true;
            }
            seen.insert(key,generations);
        }
        generations += 1;
    }
    let mut trees = 0; let mut lumb = 0;
    for y in 1..=ymax {
        for x in 1..=xmax {
            let c = grid[y*width+x];
            if c == '|' as u8 {
                trees += 1;
            }
            else if c == '#' as u8 {
                lumb += 1;
            }
        }
    }
    let p1 = trees * lumb;
    return (p1, 0);
}


fn main() {
    assert!(process(
".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.".to_owned(), 10) == (1147,0));

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1, |_| { process(input.clone(),10); }); bench_result.print_stats();

    devtime.start();
    let (part1, _p1) = process(input.clone(),10);
    let (part2, _p2) = process(input.clone(),1000000000);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}