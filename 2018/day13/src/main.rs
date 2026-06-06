//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::str;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

struct Cart {
    x:i32, y:i32, d:usize, t:usize,
}

fn disp(grid:&Vec<Vec<u8>>, carts:&Vec<Cart>)
{
//    for y in 0..grid.len() {
//        println!("{}",str::from_utf8(&grid[y]).unwrap());
//    }
    for c in 0..carts.len() {
        print!("{},{} - ",carts[c].x,carts[c].y);
    }
    println!("");
}

fn process(inp:String) -> (i32,i32,i32,i32)
{
    let mut grid:Vec<Vec<u8>> = inp.split("\n").map(|x| x.as_bytes().to_vec()).collect();
    let mut carts:Vec<Cart> = vec![];
    let rows = grid.len();
    let cols = grid[0].len();
    for y in 0..rows {
        for x in 0..cols {
            if grid[y][x] == '>' as u8 {
                carts.push(Cart{x:x as i32, y:y as i32, d:0,t:0});
                grid[y][x] = '-' as u8;
            }
            else if grid[y][x] == '^' as u8 {
                carts.push(Cart{x:x as i32, y:y as i32, d:1,t:0});
                grid[y][x] = '|' as u8;
            }
            else if grid[y][x] == '<' as u8 {
                carts.push(Cart{x:x as i32, y:y as i32, d:2,t:0});
                grid[y][x] = '-' as u8;
            }
            else if grid[y][x] == 'v' as u8 {
                carts.push(Cart{x:x as i32, y:y as i32, d:3,t:0});
                grid[y][x] = '|' as u8;
            }
        }
    }
    let dx:Vec<i32> = vec![1,0,-1,0];
    let dy:Vec<i32> = vec![0,-1,0,1];
    let dt:Vec<usize> = vec![1,0,3];
    let bslash:Vec<usize> = vec![3,2,1,0];
    let fslash:Vec<usize> = vec![1,0,3,2];
    let mut seconds:i64 = 0;
    let nr_of_carts = carts.len();
    let mut collx = -1;
    let mut colly = -1;
    let mut carts_remaining = nr_of_carts;
    loop {
//        let mut moved_carts:Vec<Cart> = vec![];
        if carts_remaining <= 1 { 
            if carts_remaining == 0 {return (collx,colly,0,0)};
            // Find last cart:
            for k in 0..nr_of_carts {
                if carts[k].x >= 0 {return (collx,colly,carts[k].x,carts[k].y);}
            }
        }
        'next_cart: for i in 0..nr_of_carts {
            let mut x = carts[i].x;
            if x < 0 {continue} // Cart has been removed!

            let mut y = carts[i].y;
            let mut d = carts[i].d;
            let mut t = carts[i].t;
            x += dx[d];
            y += dy[d];
/*             for j in 0..i {
                if x == moved_carts[j].x && y == moved_carts[j].y {
                    // Collisions!
                    return (x as i64, y as i64);
                }
            }
 */            
            for j in 0..nr_of_carts {
                if j == i {continue}
                if carts[j].x < 0 {continue}
                if x == carts[j].x && y == carts[j].y {
                    // Collisions!
//                    println!("Collision at({x},{y}) after {seconds} seconds");
                    if collx < 0 {
                        collx = x;
                        colly = y;
                    }
                    carts[i].x = -1;
                    carts[j].x = -1;
                    carts_remaining -= 2;
                    continue 'next_cart;
                }
            }
            let g = grid[y as usize][x as usize];
            if g == '+' as u8 {
                d = (d + dt[t]) & 3;
                t = if t == 2 {0}else{t+1};
            }
            else if g == '\\' as u8 {
                d = bslash[d];
            }
            else if g == '/' as u8 {
                d = fslash[d];
            }
            carts[i] = Cart{x:x,y:y,d:d,t:t};
//            moved_carts.push(Cart{x:x,y:y,d:d,t:t});
        }
        seconds += 1;
//        carts = moved_carts;
        carts.sort_by(|a,b| if a.y == b.y { a.x.partial_cmp(&b.x).unwrap() } 
            else {a.y.partial_cmp(&b.y).unwrap()});
//        disp(&grid,&carts);
    }
}


fn main() {
    assert!(process(
"/>-<\\  
|   |  
| /<+-\\
| | | v
\\>+</ |
  |   ^
  \\<->/".to_owned()).3 == 4);

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (p1x,p1y,p2x,p2y)= process(input.clone());
    devtime.stop();

    println!("Part1 = {p1x},{p1y}");
    println!("Part2 = {p2x},{p2y}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}