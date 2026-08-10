// Fastest run: Surface Pro 8
//              Acer 400 ns

//use devtimer::DevTime;
use devtimer::run_benchmark;
//use std::arch::x86_64::*;

struct Display {
    t:i64,
    x:i64,
    look:i64,
    row:i64,
    part1:i64,
    cursor:usize,
    screen:[u8;246],
}

impl Display {
    fn new() -> Display {
        Display {
            t:0,
            x:1,
            look:20,
            row:0,
            part1:0,
            cursor:0,
            screen:[b'.';246],
        }
    }
    fn tick(&mut self) {
        self.t += 1;
        if self.look == self.t {
            self.part1 += self.t * self.x as i64;
            self.look += 40;
        }
        if self.row >= self.x-1 && self.row <= self.x+1 {
            self.screen[self.cursor] = b'#';
        }
        self.cursor += 1;
        self.row += 1;
        if self.row == 40 {
            self.row = 0;
            self.screen[self.cursor] = b'\n';
            self.cursor += 1;
        }
    }

}

#[inline(never)]
fn process(inp:&str) -> Display
{
    let input = inp.as_bytes();
    let mut display = Display::new();
    let mut i = 0;
    while i < input.len() {
        match input[i] {
            b'n' => {
                display.tick();
                i += 5;
            },
            b'a' => {
                display.tick();
                display.tick();
                let mut neg = false;
                i += 5;
                if input[i] == b'-' {
                    neg = true;
                    i += 1;
                }
                let mut val = (input[i] - b'0') as i64; i += 1;
                while input[i] >= b'0' {
                    val = val * 10 + (input[i] - b'0') as i64;
                    i += 1;
                }
                if neg {
                    val = -val;
                }
                display.x += val;
                i += 1;
            },
            _ => {
                println!("Unknown instruction {}{} at offset {i}", input[i] as char, input[i]);
                panic!("aborting...");
            }
        }
    }
    display
}


fn main() {
    let mut input = std::fs::read_to_string("input.txt").unwrap();
    if input.as_bytes()[input.len() - 1] != b'\n' {
        input.push('\n');
    }

    let bench_result = run_benchmark(1000, |_| {
        process(&input);
    });
    bench_result.print_stats();

    let display = process(&input);
    println!("part1={}", display.part1);
    println!("part2=\n{}", String::from_utf8(display.screen.to_vec()).unwrap());
}
