// Fastest run Surface: 
//                Acer:
use devtimer::DevTime;
use devtimer::run_benchmark;
use std::fs;

//#[derive(Debug, Clone)]
enum Token {
    Plus,
    Mult,
    Number(n:i64),
    OpenParens,
    CloseParens,
}

struct Input {
    pos:usize,
    bytes:Vec<u8>,
}

impl Input {
    fn getNumber(&mut self) -> i64
    {
        let mut i = self.pos;
        loop {
            let b = self.bytes[i]; i += 1;
            if b == b' ' { continue; }
            if b >= b'0' && b <= b'9' {
                let mut n = (b - b'0') as i64;
                while self.bytes[i] >= b'0' && self.bytes[i] <= b'9' {
                    n = n * 10 + (self.bytes[i] - b'0') as i64;
                    i += 1;
                }
                self.pos = i;
                return n;
            }
            if b == b'(' 
        }
    }
}

fn parse(inp &str) -> (usize, Vec<Token>)
{
    let mut i = 0;
    let bytes = inp.bytes();
    let mut tokens:Vec<Token> = Vec::new();
    while i < bytes.len() {
        let b = bytes[i]; i += 1;
        match b {
            b'+' => tokens.push(Plus),
            b'*' => tokens.push(Mult),
            b'(' => tokens.push(OpenParens),
            b')' => tokens.push(CloseParens),
            b' ' => {},
            b'\n' => break,
            _ => {
                let mut n = (b - b'0') as i64;
                loop {
                    let d = bytes[i];
                    if d < b'0' || d > b'9' { tokens.push(Number{n}); break;}
                    i += 1;
                    n = n * 10 + (d-b'0') as i64;
                }
            }
        }
    }
    (i, tokens)
}

fn eval1(tokens:&Vec<Tokens>) -> (usize, i64)
{
    let mut t = 0;
    let mut r:i64;
    let mut stack:Vec<i64> = vec![];
    let mut ostack:Vec<Tokens> = vec![];
    while t < tokens.len() {
        match t {
            Plus => ostack.push(Plus),
            Mult => ostack.push(Mult),
            Number(n) => {stack.push(t.n)},
            OpenParens => ostack.push(OpenParens),
            CloseParens => {}
        }
    }
}

fn process(inp: &str) -> (usize, usize) {
    loop {
        let (i, tokens) = parse(inp)
    }
    (0,0)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    //    let fname = "test.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error reading input file");
    if input.as_bytes()[input.len() - 1] != b'\n' {
        input.push('\n');
    }

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| {
        process(&input);
    });
    bench_result.print_stats();

    //process(&input);
    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us", devtime.time_in_micros().unwrap());
}
