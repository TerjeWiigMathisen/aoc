// Acer 127 us

//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

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
            let mut fslash = false;
            let mut bslash = false;
            let (ul, ur, dl, dr) = (board[p+uplt], board[p+uprt], board[p+dnlt], board[p+dnrt]);
            if ul == b'M' && dr == b'S'{
                //drt = 1;
                fslash = true;
                if board[p+uplt+uplt] == b'X' {
                    part1 += 1;
                }
            }
            if ur == b'M' && dl == b'S'{
                //dlt = 1;
                bslash = true;
                if board[p+uprt+uprt] == b'X' {
                    part1 += 1;
                }
            }
            if dl == b'M' && ur == b'S' {
                //urt = 1;
                bslash = true;
                if board[p+dnlt+dnlt] == b'X' {
                    part1 += 1;
                }
            }
            if dr == b'M' && ul == b'S' {
                //ult = 1;
                fslash = true;
                if board[p+dnrt+dnrt] == b'X' {
                    part1 += 1;
                }
            }
            part2 += (fslash & bslash) as i32; //(drt | ult) & (dlt | urt);
            // Try 4 remaining directions
            let (u, d, l, r) = (board[p+up], board[p+dn], board[p+lt], board[p+rt]);
            if u == b'M' && d == b'S' && board[p+up+up] == b'X' {
                part1 += 1;
            }
            if d == b'M' && u == b'S' && board[p+dn+dn] == b'X' {
                part1 += 1;
            }
            if l == b'M' && r == b'S' && board[p+lt+lt] == b'X' {
                part1 += 1;
            }
            if r == b'M' && l == b'S' && board[p+rt+rt] == b'X' {
                part1 += 1;
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