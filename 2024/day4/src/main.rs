// 173 us

//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

fn _process(inp:String) -> (i32, i32)
{
    let mut board:Vec<Vec<char>> = Vec::new();
    board.push(vec!['*'; 1]);

    for line in inp.lines() {
        let l = "*".to_owned() + line + "*";
        board.push(l.chars().collect());
    }
    board[0] = vec!['*'; board[1].len()];
    board.push(vec!['*'; board[1].len()]);
    let mut part1 = 0;
    let mut part2 = 0;
    let mut y = 1;
    while y < board.len()-1 {
        let mut x = 1;
        while x < board[1].len()-1 {
            if board[y][x] == 'A' {
                let dnrt = board[y-1][x-1] == 'M' && board[y+1][x+1] == 'S';
                let dnlt = board[y-1][x+1] == 'M' && board[y+1][x-1] == 'S';
                let uprt = board[y+1][x-1] == 'M' && board[y-1][x+1] == 'S';
                let uplt = board[y+1][x+1] == 'M' && board[y-1][x-1] == 'S';
                if (dnrt | uplt) && (dnlt | uprt) {
                    part2 += 1;
                }
            }
            else if board[y][x] == 'X' {
                // up
                if board[y-1][x] == 'M' && board[y-2][x] == 'A'  && board[y-3][x] == 'S' {
                    part1 += 1;
                }
                // dn
                if board[y+1][x] == 'M' && board[y+2][x] == 'A'  && board[y+3][x] == 'S' {
                    part1 += 1;
                }
                // lt
                if board[y][x-1] == 'M' && board[y][x-2] == 'A'  && board[y][x-3] == 'S' {
                    part1 += 1;
                }
                // rt
                if board[y][x+1] == 'M' && board[y][x+2] == 'A'  && board[y][x+3] == 'S' {
                    part1 += 1;
                }
                // up-lt
                if board[y-1][x-1] == 'M' && board[y-2][x-2] == 'A'  && board[y-3][x-3] == 'S' {
                    part1 += 1;
                }
                // up-rt
                if board[y-1][x+1] == 'M' && board[y-2][x+2] == 'A'  && board[y-3][x+3] == 'S' {
                    part1 += 1;
                }
                // dn-lt
                if board[y+1][x-1] == 'M' && board[y+2][x-2] == 'A'  && board[y+3][x-3] == 'S' {
                    part1 += 1;
                }
                // dn-rt
                if board[y+1][x+1] == 'M' && board[y+2][x+2] == 'A'  && board[y+3][x+3] == 'S' {
                    part1 += 1;
                }
            }
            x += 1;
        }
        y += 1;
    }

    (part1, part2)
}

fn _processu8(inp:String) -> (i32, i32)
{
    let mut board:Vec<Vec<u8>> = Vec::new();
    board.push(vec![b'*'; 1]);

    for line in inp.lines() {
        let l = "*".to_owned() + line + "*";
        board.push(l.as_bytes().to_vec());
    }
    board[0] = vec![b'*'; board[1].len()];
    board.push(vec![b'*'; board[1].len()]);
    let mut part1 = 0;
    let mut part2 = 0;
    let mut y = 1;
    while y < board.len()-1 {
        let mut x = 1;
        while x < board[1].len()-1 {
            if board[y][x] == b'A' {
                let dnrt = board[y-1][x-1] == b'M' && board[y+1][x+1] == b'S';
                let dnlt = board[y-1][x+1] == b'M' && board[y+1][x-1] == b'S';
                let uprt = board[y+1][x-1] == b'M' && board[y-1][x+1] == b'S';
                let uplt = board[y+1][x+1] == b'M' && board[y-1][x-1] == b'S';
                if (dnrt | uplt) && (dnlt | uprt) {
                    part2 += 1;
                }
            }
            else if board[y][x] == b'X' {
                // up
                if board[y-1][x] == b'M' && board[y-2][x] == b'A'  && board[y-3][x] == b'S' {
                    part1 += 1;
                }
                // dn
                if board[y+1][x] == b'M' && board[y+2][x] == b'A'  && board[y+3][x] == b'S' {
                    part1 += 1;
                }
                // lt
                if board[y][x-1] == b'M' && board[y][x-2] == b'A'  && board[y][x-3] == b'S' {
                    part1 += 1;
                }
                // rt
                if board[y][x+1] == b'M' && board[y][x+2] == b'A'  && board[y][x+3] == b'S' {
                    part1 += 1;
                }
                // up-lt
                if board[y-1][x-1] == b'M' && board[y-2][x-2] == b'A'  && board[y-3][x-3] == b'S' {
                    part1 += 1;
                }
                // up-rt
                if board[y-1][x+1] == b'M' && board[y-2][x+2] == b'A'  && board[y-3][x+3] == b'S' {
                    part1 += 1;
                }
                // dn-lt
                if board[y+1][x-1] == b'M' && board[y+2][x-2] == b'A'  && board[y+3][x-3] == b'S' {
                    part1 += 1;
                }
                // dn-rt
                if board[y+1][x+1] == b'M' && board[y+2][x+2] == b'A'  && board[y+3][x+3] == b'S' {
                    part1 += 1;
                }
            }
            x += 1;
        }
        y += 1;
    }

    (part1, part2)
}

fn process_linu8(inp:String) -> (i32, i32)
{
    let mut width = 0;
    while inp.as_bytes()[width] != b'\n' {
        width += 1;
    }
    width += 1;
    let lt = usize::MAX;
    let rt = 1;
    let up = 0-width;
    let dn = width;
    let uprt = up + rt;
    let uplt = up + lt;
    let dnrt = dn + rt;
    let dnlt = dn + lt;

    let mut board:Vec<u8> = vec![b'*'; dnrt];
    board.append(&mut Vec::from(inp.as_bytes()));
    board.append(&mut vec![b'*'; dnrt]);

    let mut part1 = 0;
    let mut part2 = 0;

    let mut p = dnrt+dnrt;
    let lim = board.len()-dnrt-dnrt;
    while p < lim {
        if board[p] == b'A' {
            // Diagonals: dnrt, dnlt, uprt, uplt
            let mut drt = 0; 
            let mut dlt = 0; 
            let mut urt = 0; 
            let mut ult = 0;
            if board[p+uplt] == b'M' && board[p+dnrt] == b'S'{
                drt = 1;
                if board[p+uplt+uplt] == b'X' {
                    part1 += 1;
                }
            }
            if board[p+uprt] == b'M' && board[p+dnlt] == b'S'{
                dlt = 1;
                if board[p+uprt+uprt] == b'X' {
                    part1 += 1;
                }
            }
            if board[p+dnlt] == b'M' && board[p+uprt] == b'S' {
                urt = 1;
                if board[p+dnlt+dnlt] == b'X' {
                    part1 += 1;
                }
            }
            if board[p+dnrt] == b'M' && board[p+uplt] == b'S' {
                ult = 1;
                if board[p+dnrt+dnrt] == b'X' {
                    part1 += 1;
                }
            }
            part2 += (drt | ult) & (dlt | urt);
            // Try 4 remaining directions
            for dir in [up, dn, lt, rt].iter() {
                if board[p+dir] == b'M' && board[p+dir+dir] == b'X'  && board[p-dir] == b'S' {
                    part1 += 1;
                }
            }
        }
        p += 1;
    }
    (part1, part2)
}

fn _inner_thread(board:Vec<u8>, strt:usize, stop:usize, up:usize, dn:usize,lt:usize,
    rt:usize,uplt:usize,uprt:usize,dnlt:usize,dnrt:usize) -> (i32, i32)
{
    let mut part1 = 0;
    let mut part2 = 0;
    let mut p = strt;
    while p < stop {
        if board[p] == b'A' {
            let drt = board[p+uplt] == b'M' && board[p+dnrt] == b'S';
            let dlt = board[p+uprt] == b'M' && board[p+dnlt] == b'S';
            let urt = board[p+dnlt] == b'M' && board[p+uprt] == b'S';
            let ult = board[p+dnrt] == b'M' && board[p+uplt] == b'S';
            if (drt | ult) && (dlt | urt) {
                part2 += 1;
            }
        }
        else if board[p] == b'X' {
            // Try all 8 directions
            for dir in [up, dn, lt, rt, uplt, uprt, dnlt, dnrt].iter() {
                if board[p+dir] == b'M' && board[p+dir+dir] == b'A'  && board[p+dir+dir+dir] == b'S' {
                    part1 += 1;
                }
            }
        }
        p += 1;
    }
    (part1, part2)
}

fn _process_threads(inp:String) -> (i32, i32)
{
    let mut width = 0;
    while inp.as_bytes()[width] != b'\n' {
        width += 1;
    }
    width += 1;
    let lt = usize::MAX;
    let rt = 1;
    let up = 0-width;
    let dn = width;
    let uprt = up + rt;
    let uplt = up + lt;
    let dnrt = dn + rt;
    let dnlt = dn + lt;

    let mut board:Vec<u8> = vec![b'*'; dnrt];
    board.append(&mut Vec::from(inp.as_bytes()));
    board.append(&mut vec![b'*'; dnrt]);

    let mut part1 = 0;
    let mut part2 = 0;

    let (tx, rx) = std::sync::mpsc::channel();
    let mut handles = vec![];
    let num_threads = 8;
    let chunk_size = (board.len()-dnrt*2+num_threads-1) / num_threads;
    for i in 0..num_threads {
        let tx = tx.clone();
        let board = board.clone();
        let up = up;
        let dn = dn;
        let lt = lt;
        let rt = rt;
        let uplt = uplt;
        let uprt = uprt;
        let dnlt = dnlt;
        let dnrt = dnrt;
        let strt = i * chunk_size + dnrt;
        let stop = if i == num_threads-1 {board.len()-dnrt} else {strt + chunk_size};
        let handle = std::thread::spawn(move || {
            tx.send(_inner_thread(board, strt, stop, up, dn, lt, rt, uplt, uprt, dnlt, dnrt)).unwrap();
        });
        handles.push(handle);
    }
    for _ in handles {
        let (p1, p2) = rx.recv().unwrap();
        part1 += p1;
        part2 += p2;
    }
    (part1, part2)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process_linu8(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process_linu8(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}