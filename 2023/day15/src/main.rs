// day15
// Surface: 105.0 us
// Acer:     68.9 us

use std::fs;
use devtimer::run_benchmark;

fn hash(b:u8, hash: usize) -> usize
{
    let mut hash = hash;
    hash = (hash + b as usize) * 17 % 256;
    hash
}

#[derive(Clone, Copy)]
struct Tag {
    o:&'static str,
    val:usize,
}

#[derive(Clone)]
struct Onebox {
    tags:Vec<Tag>,
}
impl Onebox {
    fn new() -> Onebox
    {
        Onebox {tags:vec![]}
    }
}

fn process(inp:&'static str) -> (usize, usize)
{
    let mut i:usize = 0;
    let mut part1 = 0;
    let mut part2 = 0;
    let mut boxes:[Onebox;256] = core::array::from_fn(|_| Onebox::new());
    let input = inp.as_bytes();
    while i < input.len() {
//        println!("At {i}, next input: {}", input[i] as char);
        let mut h2 = 0;
        let start = i;
        let mut b:u8;
        loop {
            b = input[i]; i += 1;
            if b == b'-' || b == b'=' { break; }
            h2 = hash(b, h2);
        }
        let op = b;
        let mut h1 = hash(op, h2);
        let tag = &inp[start..i-1]; // Use the slice in place
        let tags = &mut boxes[h2].tags;
        if op == b'-' { // Must be '-', remove matching lens if found
            for t in 0..tags.len() {
                if tag == tags[t].o {
                    tags.remove(t);
                    break;
                }
            }
        }
        else { // op == b'='
            let v = input[i]; i += 1;
            h1 = hash(v, h1);
            let v = (v - b'0') as usize;
            let mut replaced = false;
            for t in 0..tags.len() {
                if tag == tags[t].o {
                    tags[t].val = v;
                    replaced = true;
                    break;
                }
            }
            if !replaced { tags.push(Tag{o:tag, val:v})}
        }
        i += 1; // skip comma
        part1 += h1;
    }
    for b in 1..=256 {
        let tags = &boxes[b-1].tags;
        for slot in 1..=tags.len() {
            let focus = b*slot*tags[slot-1].val;
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

    let input:&'static str = Box::leak(input.into_boxed_str());

    let bench_result = run_benchmark(1000, |_| {
        process(&input);
    });
    bench_result.print_stats();

    let res = process(input);
    println!("part1={}\npart2={}", res.0, res.1);
}
