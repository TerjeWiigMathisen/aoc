// day13
// Surface: 52.8 us
// Acer:   

use std::fs;
use devtimer::run_benchmark;

// return new i value, xmax, bit matrix
fn parse_block(inp:&[u8], i:usize) -> (usize, usize, Vec<u64>)
{
    let mut i = i;
    let mut out:Vec<u64> = Vec::new();
    let mut xmax;
    loop {
        let mut x = 0;
        let mut xlen = 0;
        loop {
            let b = inp[i]; i += 1;
            if b == b'\n' {
                out.push(x);
                //if xlen > 0 {out.push(x)};
                x = 0;
                xmax = xlen;
                xlen = 0;
                if i >= inp.len() || inp[i] == b'\n' { return (i+1, xmax, out)}
                continue;
            }
            let bit = ((b & 1) as u64) << xlen;
            x += bit;
            xlen += 1;
        }
    }
}

fn mirror(out:&[u64], p1:usize, p2:usize) -> (usize, usize)
{
    let mut res:[usize;2] = [p1, p2];
    for i in 1..out.len() {
        let mut diff = (out[i] ^ out[i-1]).count_ones();
        if diff <= 1 {
            let mut up = i-1;
            let mut dn = i+1;
            let mut mirror = true;
            while up > 0 && dn < out.len() {
                diff += (out[up-1] ^ out[dn]).count_ones();
                if diff > 1 {
                    mirror = false;
                    break;
                }
                up -= 1; dn += 1;
            }
            if mirror { 
                res[diff as usize] = i;
                if res[0] != 0 && res[1] != 0 {return (res[0],res[1])}
            }
        }
    }
    (res[0],res[1])
}

fn transpose(inp:&[u64], xlen:usize) -> Vec<u64>
{
    let mut out:Vec<u64> = Vec::new();
    for y in 0..xlen {
        let mut bits = 0;
        for x in 0..inp.len() {
            let b = (inp[x] >> y) & 1;
            bits += b << x;
        }
        out.push(bits);
    }
    out
}

fn process(inp:&str)->(usize,usize, usize, usize)
{
    let input = inp.as_bytes();
    let mut i = 0;
    let mut part1 = 0;
    let mut part2 = 0;
    let mut blocks = 0;
    let mut transposes = 0;
    loop {
        let (o, xmax, out) = parse_block(&input, i);
        blocks += 1;
        i = o;
        let (p1,p2) = mirror(&out, 0, 0);
        part1 += p1 * 100; part2 += p2 * 100;
        if p1 == 0 || p2 == 0 {
            let tra = transpose(&out, xmax);
            transposes += 1;
            let (p3,p4) = mirror(&tra, p1, p2);
            if p1 == 0 { part1 += p3; } 
            if p2 == 0 { part2 += p4; }
        }
        if i >= input.len() {break;}
    }
    (part1, part2, blocks, transposes)
}

fn main() {
//    let fname = "test.txt"; // instead of args[1]
    let fname = "input.txt"; // instead of args[1]
    let input = fs::read_to_string(fname).expect("Error readin input file");

    let bench_result = run_benchmark(1000, |_| {
        process(&input);
    });
    bench_result.print_stats();

    let res = process(&input);
    println!("part1={}\npart2={}\n{} total blocks, {} transposes",
        res.0, res.1, res.2, res.3);
}
