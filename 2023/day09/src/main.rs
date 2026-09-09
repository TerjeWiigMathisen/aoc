// day09
// Surface:
// Acer:    31.4 us
use std::fs;
use devtimer::run_benchmark;

fn parsei64(inp:&[u8], i:usize, out:&mut Vec<i64>) -> usize
{
    let mut i = i;
    out.clear();
    loop {
        loop {
            let mut sign = 1;
            if inp[i] == b'-' { sign = -1; i += 1; }
            let mut n = (inp[i] - b'0') as i64; i += 1;
            while inp[i] >= b'0' /*  && inp[i] <= b'9' */ {
                let d = (inp[i] - b'0') as i64;
                n = n * 10 + d;
                i += 1;
            }
            if sign == -1 { n = -n;}
//            n *= sign;
            out.push(n);
            if inp[i] == b'\n' {return i + 1;}
            i += 1; // skip space
        }
    }
}

pub fn process(inp:&String)->(i64, i64)
{
    let input = inp.as_bytes();
    let mut i = 0;
    let mut nums:Vec<i64> = Vec::new();
    let mut first:Vec<i64> = Vec::new();
    let mut diffs:Vec<i64> = Vec::new();
    let mut part1 = 0;
    let mut part2 = 0;
    loop {
        i = parsei64(input, i, &mut nums);
//        let mut last:Vec<i64> = Vec::new();
        first.clear();
        let mut pred = 0;
        diffs.clear();
        loop {
            let mut sumdiff = 0;
            let mut curr = nums[0];
            first.push(curr);
            for j in 1..nums.len() {
                let diff = nums[j] - curr;
                sumdiff |= diff;
                diffs.push(diff);
                curr = nums[j];
            }
//            last.push(curr);
            pred += curr;
            let t = nums;
            nums = diffs;
            diffs = t;
            diffs.clear();
            if sumdiff == 0 { break; }
        }
        let mut j = first.len();
        let mut before = 0;
        while j > 0 {
            j -= 1;
            before = first[j] - before;
        };
        part1 += pred;
        part2 += before;

        if i >= input.len()  { break; }
    }
    return (part1, part2);
}

fn _p1k(input:&String)->usize
{
    for _ in 0..1000 {
        process(input);
    }
    return 0;
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
//    let fname = "test.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error reading input file");
    if input.as_bytes()[input.as_bytes().len()-1] != b'\n' as u8 {input.push('\n');}

    let bench_result = run_benchmark(1000, |_| {
        process(&input);
    });
    bench_result.print_stats();

    let res = process(&input);
    println!("part1={}\npart2={}", res.0, res.1);
}
