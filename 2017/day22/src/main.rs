//use std::io;
//use std::env;
use std::fs;
//use substring::Substring;
//use std::collections::HashMap;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;

// fn dump(board:&HashMap<String,u8>, cx:i32, cy:i32, dir:i32)
// {
//     let mut minx = i32::MAX;
//     let mut maxx = i32::MIN;
//     let mut miny = i32::MAX;
//     let mut maxy = i32::MIN;
//     for key in board.clone().into_keys() {
//         let parts:Vec<i32> = key.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
//         let (x,y) = (parts[0],parts[1]);
//         if x < minx {minx = x; }
//         if x > maxx {maxx = x; }
//         if y < miny {miny = y; }
//         if y > maxy {maxy = y; }
//     }
//     let dirc: Vec<char> = vec!['>','^','<','v'];
//     for y in miny..=maxy {
//         for x in minx..=maxx {
//             let key = format!("{x},{y}");
//             let mut c:char;
//             if board.contains_key(&key) && board[&key] == '#' as u8 {
//                 c = '#';
//             }
//             else {
//                 c = '.';
//             }
//             if x == cx && y == cy {
//                 c = dirc[dir as usize];
//             }
//             print!("{c}");
//         }
//         println!("");
//     }
//     println!("");
// }

const OFFS:usize = 240;

fn process(inp:String, iterations:usize) -> u32
{
    let mut count = 0;
    let input = inp.clone();
    let mut board:Vec<Vec<u8>> = vec![vec![b'.';500];500];

    let start:Vec<Vec<u8>> = input.
        split("\n").
        map(|x| x.as_bytes().to_owned()).
        collect();
    for y in 0..start.len() {
        for x in 0..start[y].len() {
            board[y+OFFS][x+OFFS] = start[y][x];
        }
    }
    let mut x = (start[0].len()/2 + OFFS) as i32;
    let mut y = (start.len()/2 + OFFS) as i32;
    let mut dir:usize = 1;
//    dump(&board,x,y,dir);

    let dx:[i32;4] = [1,0,-1,0];
    let dy:[i32;4] = [0,-1,0,1];
    for _ in 0..iterations {
        if board[y as usize][x as usize] == b'#' {
            board[y as usize][x as usize] = b'.';
            dir = (dir+3) & 3; // dir--
        }
        else {
            board[y as usize][x as usize] = b'#';
            dir = (dir+1) & 3;
            count += 1;
        }
        x += dx[dir]; 
        y += dy[dir];
//        dump(&board,x,y,dir);
    }
//    dump(&board,x,y,dir);
    return count;
}

fn process2(inp:String, iterations:usize) -> u32
{
    let mut count = 0;
    let input = inp.clone();
    let mut board:Vec<Vec<u8>> = vec![vec![0;500];500];

    // let mut minx = i32::MAX;
    // let mut maxx = i32::MIN;
    // let mut miny = i32::MAX;
    // let mut maxy = i32::MIN;

    let start:Vec<Vec<u8>> = input.
        split("\n").
        map(|x| x.as_bytes().to_owned()).
        collect();
    for y in 0..start.len() {
        for x in 0..start[y].len() {
            board[y+OFFS][x+OFFS] = 2*(start[y][x] == b'#') as u8;
        }
    }
    let mut x:i32 = ((start[0].len())/2 + OFFS) as i32;
    let mut y:i32 = ((start.len())/2 + OFFS) as i32;
    let mut dir:usize = 1;

    let dx:[i32;4] = [1,0,-1,0];
    let dy:[i32;4] = [0,-1,0,1];
    //let nextc:[u8;4] = [1,2,3,0];
    let ddir:[usize;4] = [1,0,3,2];
    let dcount:[u32;4] = [0,1,0,0];
    for _ in 0..iterations {
        let mut c = board[y as usize][x as usize];
        dir = (dir+ddir[c as usize]) & 3;
        count += dcount[c as usize];
        c = (c+1)&3; // nextc[c as usize];
        board[y as usize][x as usize] = c;
        x += dx[dir]; 
        y += dy[dir];
    }
    return count;
}

fn main() {

    assert!(process2(
"..#
#..
...".to_owned(),100) == 26);
    let fname = "input.txt"; // instead of args[1]
    let input = fs::read_to_string(fname).expect("Error readin input file");
    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10, |_| {
        process(input.clone(),10000);
        process2(input.clone(),10000000);
    });
    bench_result.print_stats();

    devtime.start();
    let part1 = process(input.clone(),10000);
    let part2 = process2(input.clone(),10000000);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}