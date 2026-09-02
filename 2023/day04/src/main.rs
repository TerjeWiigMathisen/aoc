// day04
// Surface:  15.4 us
// Acer:     
use std::fs;
use devtimer::run_benchmark;

pub fn process(inp:&String)->(usize, usize)
{
    let input = inp.as_bytes();
    let ilen = input.len();
    let mut i = 0;
    let mut c:usize = 0;
    let mut part1 = 0;
    let mut part2 = 0;
    let mut copies:Vec<usize> = vec![0;250];
    loop {
        i += 10;
//        while copies.len() <= c {copies.push(0);}
        copies[c] += 1;
        let mut winners:[u8;100] = [0;100];
        loop {
            let widx = (input[i] & 15)*10 + (input[i+1] & 15);
            winners[widx as usize] = 1;
            i += 3;
            if input[i] == b'|' {break;}
        }
        i += 2;
        let mut price:usize = 0;
        loop {
            let tidx = (input[i] & 15)*10 + (input[i+1] & 15);
            price += winners[tidx as usize] as usize;
            i += 3;
            if input[i-1] == b'\n' {break;}
        }

        if price > 0 {
            part1 += 1 << (price-1);
//            while c+price >= copies.len() {copies.push(0)}
            let curr = copies[c];
            for j in 1..=price {
                copies[c+j] += curr;
            }
        }
        c += 1;
//        println!("At offset {i}, {c} cards");
        if i >= ilen {break}
    }
    for i in 0..copies.len() {
        part2 += copies[i];
    }
    return (part1, part2);
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] != b'\n' as u8 {input.push('\n');}

    let bench_result = run_benchmark(1000, |_| {
        process(&input);
    });
    bench_result.print_stats();

    let res = process(&input);
    println!("part1={}\npart2={}", res.0, res.1);
}
