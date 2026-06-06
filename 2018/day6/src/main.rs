//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;
//use priority_queue::PriorityQueue;

#[derive(Clone)]
struct Pos {
    x:i32,
    y:i32,
    name:i32,
    manh:i32,
}

fn dump(grid:Vec<Vec<Pos>>)
{
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let nm = grid[y][x].name;
            let mut c:char = '-'; // Assume nm < 0
            if nm == 0 {c = '.';}
            else if nm > 0 { c = (nm - 1 + 'A' as i32) as u8 as char;}
            print!("{c}");
        }
        println!("");
    }
    println!("");
}

fn sumdist(x:i32, y:i32, posvec:&Vec<Pos>) -> i32
{
    let mut sum = 0;
    for p in posvec {
        sum += (p.x-x).abs() + (p.y-y).abs();
    }
    return sum;
}

fn process(inp:String, maxdist:i32) -> (i64,i64)
{
    let lines:Vec<String> = inp.split("\n").map(|s| s.to_string()).collect();
    let mut names = 0;
    let p = parser!(x0:i32 ", " y0:i32 =>
        Pos{x:x0, y:y0, name:0, manh:0 });

    let mut posvec:Vec<Pos> = vec![];
    let mut hitborder:Vec<bool> = vec![false];
    let (mut xmin, mut ymin, mut xmax, mut ymax) = (i32::MAX,i32::MAX,i32::MIN,i32::MIN);
    let mut xsum = 0; let mut ysum = 0;
    for li in lines {
        let mut point = p.parse(&li).unwrap();
        names += 1;
        point.name = names;
        let x = point.x;
        let y = point.y;
        if x < xmin {xmin = x;}
        if x > xmax {xmax = x;}
        if y < ymin {ymin = y;}
        if y > ymax {ymax = y;}
        xsum += x; ysum += y;
        posvec.push(point);
        hitborder.push(false);
    }
    assert!(xmin >= 1 && ymin >= 1);
    // The grid will have at least one cell as guard around the perimeter, in order
    // to simplify the detection of areas that grow to infinity
    let mut grid = vec![vec![Pos{x:0,y:0,name:0,manh:0};(xmax+2)as usize];(ymax+2)as usize];
    let plen = posvec.len() as i32;
    let x0 = (xsum + (plen>>1)) / plen;
    let y0 = (ysum + (plen>>1)) / plen;
    let mut xleft = x0 - maxdist / plen - 1;
    let mut xright = x0 + maxdist / plen + 1;
    while sumdist(xleft,y0,&posvec) >= maxdist {
        xleft += 1;
        if xleft >= xright {panic!("Zero area!")}
    }
    while sumdist(xright,y0,&posvec) >= maxdist {
        xright -= 1;
    }
    let mut p2 = (xright-xleft+1) as i64;
    for dir in [-1,1].iter() {
        let (xl,xr) = (xleft, xright);
        let mut y = y0;
        'outer: loop {
            y += dir;
            while sumdist(xleft,y,&posvec) >= maxdist {
                xleft += 1;
                if xleft >= xright {break 'outer;}
            }
            while sumdist(xright,y,&posvec) >= maxdist {
                xright -= 1;
            }
            p2 += (xright-xleft+1) as i64;
        }
        xleft = xl; xright=xr;
    }
 
    let mut manhattan = 0;
    loop {
//        dump(grid.clone());
//       println!("manhattan:{manhattan},posvec.len={}",posvec.len());
        let mut pv:Vec<Pos> = vec![];
        let mut fill = 0;
        for i in 0..posvec.len() {
            let x = posvec[i].x;
            let y = posvec[i].y;
            let nm = posvec[i].name;
//            if hitborder[nm as usize] {continue;}
//            if x < xmin || x > xmax || y < ymin || y > ymax {
//                hitborder[nm as usize] = true;
//                continue;
//            }
            if grid[y as usize][x as usize].name == 0 {
                grid[y as usize][x as usize].name = nm;
                grid[y as usize][x as usize].manh = manhattan;
                fill += 1;
            }
            else if grid[y as usize][x as usize].name == nm {
                // reached twice from same source
                posvec[i].name = -2; // Block for second stage!
                continue;
            }
            else if grid[y as usize][x as usize].manh == manhattan { // Equal distance from multiple, skip!
                grid[y as usize][x as usize].name = -1;
            }
        }
        manhattan += 1;
        for i in 0..posvec.len() {
            let x0 = posvec[i].x;
            let y0 = posvec[i].y;
            let nm = posvec[i].name;
            if nm <= 0 {continue;}
            if x0 < xmin || x0 > xmax || y0 < ymin || y0 > ymax {
                if nm == grid[y0 as usize][x0 as usize].name {
                    hitborder[nm as usize] = true;
                }
                continue;
            }
            if grid[y0 as usize][x0 as usize].name == nm { // No collision here!
                pv.push(Pos{x:x0,y:y0-1,name:nm,manh:manhattan});
                pv.push(Pos{x:x0-1,y:y0,name:nm,manh:manhattan});
                pv.push(Pos{x:x0+1,y:y0,name:nm,manh:manhattan});
                pv.push(Pos{x:x0,y:y0+1,name:nm,manh:manhattan});
            }
        }
        posvec = pv;
        if fill == 0 {break;}
    }
    // Find largest internal area
    let mut area:Vec<i64> = vec![0;hitborder.len()];
    for y in (ymin as usize)..=(ymax as usize) {
        for x in (xmin as usize)..=(xmax as usize) {
            let nm = grid[y][x].name;
            if nm > 0 && !hitborder[nm as usize] {
                area[nm as usize] += 1;
            }
        }
    }
    area.sort();
    return (area[area.len()-1],p2);
}


fn main() {
    assert!(process(
"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9".to_owned(),32) == (17,16));

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10, |_| { process(input.clone(),10000); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input.clone(),10000);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}