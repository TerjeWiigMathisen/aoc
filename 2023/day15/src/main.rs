// day15
// Surface: 105.0 us
// Acer:     36.1 us

use std::fs;
use devtimer::run_benchmark;

fn hash(b:u8, hash: usize) -> usize
{
    let mut hash = hash;
    hash = (hash + b as usize) * 17 % 256;
    hash
}

#[derive(Clone, Copy)]
struct Onebox {
    tags:[u32;8],
    vals:[u32;8],
}
impl Onebox {
    fn new() -> Onebox
    {
        Onebox {tags:[0;8],vals:[0;8]}
    }
}

fn process(inp:&str) -> (usize, usize)
{
    let mut i:usize = 0;
    let mut part1 = 0;
    let mut part2 = 0;
    let mut boxes:[Onebox;256] = [Onebox::new();256];
    let input = inp.as_bytes();
    while i < input.len() {
//        println!("At {i}, next input: {}", input[i] as char);
        let mut h2 = 0;
        let mut b:u8;
        let mut tag:u32 = 0;
        loop {
            b = input[i]; i += 1;
            if b == b'-' || b == b'=' { break; }
            h2 = hash(b, h2);
            assert!(b >= b'a' && b <= b'z');
            let t = (b & 31) as u32;
            tag = (tag << 5) | t;
        }
        let op = b;
        let mut h1 = hash(op, h2);
        let tags = &mut boxes[h2].tags;
        let vals = &mut boxes[h2].vals;
        if op == b'-' { // Must be '-', remove matching lens if found
            for t in 0..tags.len() {
                if tag == tags[t] {
                    for j in t+1..tags.len() {
                        tags[j-1] = tags[j];
                        vals[j-1] = vals[j];
                        if vals[j] == 0 { break; }
                    }
                    break;
                }
            }
        }
        else { // op == b'='
            let v = input[i]; i += 1;
            h1 = hash(v, h1);
            let v = (v - b'0') as u32;
            for t in 0..tags.len() {
                if tag == tags[t] {
                    vals[t] = v;
                    break;
                }
                if vals[t] == 0 {
                    tags[t] = tag;
                    vals[t] = v;
                    break;
                }
            }
        }
        i += 1; // skip comma
        part1 += h1;
    }
    for b in 1..=256 {
        let vals = &boxes[b-1].vals;
        for slot in 1..=vals.len() {
            let v = vals[slot-1] as usize;
            if v == 0 { break; }
            let focus = b*slot*v;
            part2 += focus;
        }
    }
    (part1, part2)
}

fn main() {
//    let fname = "test.txt"; // instead of args[1]
    let fname = "input.txt"; // instead of args[1]
    let mut input:String = fs::read_to_string(fname).expect("Error reading input file");
    if input.ends_with('\n') { input.pop(); }
    if !input.ends_with(',') { input.push(','); }

//    let input:&'static str = Box::leak(input.into_boxed_str());

    let bench_result = run_benchmark(1000, |_| {
        process(&input);
    });
    bench_result.print_stats();

    let res = process(&input);
    println!("part1={}\npart2={}", res.0, res.1);
}
