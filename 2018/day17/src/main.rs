// use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

fn _disp(grid:&Vec<u8>, width:usize, xmin:usize, ymin:usize, xmax:usize, ymax:usize)
{
    for y in ymin-1..=ymax+1 {
        for x in xmin-1..=xmax+1 {
            print!("{}", grid[y*width+x] as char);
        }
        println!("");
    }
    println!("grid from ({xmin},{ymin}) to ({xmax},{ymax})");
}

fn fill(grid:&mut Vec<u8>, width:usize, spos:usize, xmin:usize,ymin:usize,xmax:usize,ymax:usize)->usize
{
    let mut pos = spos;
    //let mut x = sx; let mut y = sy;
    while grid[pos] == '.' as u8 {
        grid[pos] = '|' as u8;
        pos += width;
//        y += 1;
    }
    if grid[pos] == '|' as u8 { 
        // We either hit the guard bottom, or another path has already been here
        return 0;
    }
    pos -= width; // We must stay away from the floor!
    // Look left
    let mut below = pos+width;
    loop {
        let mut lbelow = below-1;
        let mut lpos = pos-1;
        while (grid[lpos] == b'.' || grid[lpos] == b'|') && (grid[lbelow] == b'#' || grid[lbelow] == b'~') {
            lpos -= 1;
            lbelow -= 1;
        }
        let mut rbelow = below+1;
        let mut rpos = pos+1;
        while (grid[rpos] == b'.' || grid[rpos] == b'|') && (grid[rbelow] == b'#' || grid[rbelow] == b'~') {
            rpos += 1;
            rbelow += 1;
        }
        if grid[lpos] == b'#' && grid[rpos] == b'#' {
            for p in lpos+1..rpos {
                grid[p] = b'~';
            }
            below = pos;
            pos -= width; // Fill layer above!
        }
        else {
            for p in lpos+1..rpos {
                grid[p] = b'|';
            }
            if grid[lpos] == b'.' {
//                disp(&grid,width,xmin,ymin,xmax,ymax);
                fill(grid, width, lpos,xmin,ymin,xmax,ymax);
            }
            if grid[rpos] == b'.' {
//                disp(&grid,width,xmin,ymin,xmax,ymax);
                fill(grid, width, rpos,xmin,ymin,xmax,ymax);
            }
            break;
        }
    }
//    disp(&grid,width,xmin,ymin,xmax,ymax);
    return 0;
}

#[derive(Clone)]
struct HLine {
  x0:u32,
  x1:u32,
  y:u32,
}
#[derive(Clone)]
struct VLine {
    x:u32,
    y0:u32,
    y1:u32,
}

fn parse3(inp:&str) -> (bool, [u32;3])
{
    let inpb = inp.as_bytes();
    let vert = inpb[0] == b'x';
    let mut n:[u32;3] = [0;3];
    let mut i = 1;
    for j in 0..3 {
        while inpb[i] < b'0' || inpb[i] > b'9' { i += 1; }
        let mut r:u32 = 0;
        while i < inpb.len() && inpb[i] >= b'0' && inpb[i] <= b'9' {
            r = r*10 + (inpb[i] & 15) as u32;
            i += 1;
        }
        n[j] = r;
    }
    (vert, n)
}

fn process(inp:&str) -> (i64,i64)
{
    let lines = inp.lines();
    let mut ymin = u32::MAX;
    let mut ymax = 0;
    let mut xmin = u32::MAX;
    let mut xmax = 0;
    let mut vl:Vec<VLine> = vec![];
    let mut hl:Vec<HLine> = vec![];
    for li in lines {
        let (v, n) = parse3(&li);
        if v {
            let (x, y0, y1) = (n[0], n[1], n[2]);
            vl.push(VLine{x:x,y0:y0,y1:y1});
            if x < xmin { xmin = x;}
            else if x > xmax { xmax = x;}
            if y0 < ymin { ymin = y0;}
            else if y1 > ymax { ymax = y1;}
        }
        else {
            let (y, x0, x1) = (n[0], n[1], n[2]);
            hl.push(HLine{y:y,x0:x0,x1:x1});
            if x0 < xmin { xmin = x0;}
            else if x1 > xmax { xmax = x1;}
            if y < ymin { ymin = y;}
            else if y > ymax { ymax = y;}
        }
    }
//    println!("xmin = {xmin}, xmax = {xmax}, ymin = {ymin}, ymax = {ymax}");
    let xoffset = xmin-2;
    let width = xmax+2 - xoffset;
    let height = ymax+4;
    let mut grid:Vec<u8> = vec!['.' as u8;(width*height) as usize];
    for vline in vl {
        for y in vline.y0..=vline.y1 {
            grid[(y*width + vline.x - xoffset) as usize] = b'#';
        }
    }
    for hline in hl {
        for x in hline.x0..=hline.x1 {
            grid[(hline.y*width + x - xoffset) as usize] = b'#';
        }
    }
    for p in width*(height-1)..width*height {
        grid[p as usize] = '|' as u8; // Guard layer below bottom
    }
//    disp(&grid, width,xmin,ymin,xmax,ymax);
    fill(&mut grid, width as usize, 500 - xoffset as usize, xmin as usize, ymin as usize, xmax as usize, ymax as usize);
    let mut p1 = 0;
    let mut p2 = 0;
    for y in ymin..=ymax {
        for x in xmin-1..=xmax+1 {
            let u = grid[(y*width+x-xoffset) as usize];
            if u == b'|' { p1 += 1;}
            if u == b'~' { p2 += 1;}
        }
    }
    p1 += p2;
//    println!("part1 = {p1}");
//    disp(&grid, width,xmin,ymin,xmax,ymax);
return (p1, p2);
}

fn main() {

     assert!(process(
"x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504") == (57,29));

     assert!(process(
"x=500, y=2..3
x=501, y=5..6
y=7, x=501..502
x=504, y=1..6
y=9, x=503..504") == (31,0));

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10, |_| { process(&input); }); bench_result.print_stats();

//    panic!("Stop now!");

    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}