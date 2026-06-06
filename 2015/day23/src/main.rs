// Fastest run 69 us without 3xp1 patern matching
// 1.5 us with compiled code for 3xp1 counting

use std::env;
use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;

fn _run(a_start:i32, lines:&Vec<&str>) -> i32
{
    let mut ip:i32 = 0;
    let mut ab:[i32;2] = [a_start,0];
    let maxip:i32 = lines.len() as i32;
    while ip < maxip {
        let line = lines[ip as usize];
//        println!("a:{}, b:{}, ip:{}, line:{}", ab[0], ab[1], ip, line);
        ip += 1;
        let parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let op = parts[0];
        let var = parts[1];
        match op {
            "inc" => {
                ab[(var == "b") as usize] += 1;
            },
            "tpl" => {
                ab[(var == "b") as usize] *= 3;
            },
            "hlf" => {
                ab[(var == "b") as usize] >>= 1;
            },
            "jmp" => {
                ip += var.parse::<i32>().unwrap()-1;
            },
            "jie" => {
                if ab[(var == "b,") as usize] & 1 == 0 {
                    ip += parts[2].parse::<i32>().unwrap()-1;
                }
            },
            "jio" => { // Jump if One
                if ab[(var == "b,") as usize] == 1 {
                    ip += parts[2].parse::<i32>().unwrap()-1;
                }
            },
            _ => { assert!(false); }
        }
    }
    ab[1]
}

fn runab(a_start:u64, lines:&Vec<&str>) -> i32
{
    let mut ip:i32 = 0;
    let mut a = a_start;
    let mut b = 0;
    let maxip:i32 = lines.len() as i32;
    while ip < maxip {
        let line = lines[ip as usize];
//        println!("a:{}, b:{}, ip:{}, line:{}", ab[0], ab[1], ip, line);
        ip += 1;
        match line {
            "inc a" => { a += 1; },
            "inc b" => { b += 1; },
            "tpl a" => { a *= 3; },
            "tpl b" => { b *= 3; },
            "hlf a" => { a >>= 1; },
            "hlf b" => { b >>= 1; },
            "jio a, +8" => {
                if ip+7 == maxip &&
                "inc b" == lines[ip as usize] &&
                "jie a, +4" == lines[ip as usize+1] &&
                "tpl a" == lines[ip as usize+2] &&
                "inc a" == lines[ip as usize+3] &&
                "jmp +2" == lines[ip as usize+4] &&
                "hlf a" == lines[ip as usize+5] &&
                "jmp -7" == lines[ip as usize+6] {
                    while a != 1 {
                        if a & 1 == 1 {
                            b += 1;
                            a = a*3+1;
                        }
                        b += 1;
                        a >>= 1;
                    }
                    return b;
                }
                if a == 1 {
                    ip += 7;
                }
            },
            _ => {
                let parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
                let op = parts[0];
                let var = parts[1];
                match op {
                    "jmp" => {
                        ip += var.parse::<i32>().unwrap()-1;
                    },
                    "jie" => {
                        let branchip:i32 = ip + parts[2].parse::<i32>().unwrap()-1;
                        if var == "b," {
                            if b & 1 == 0 {
                                ip = branchip;
                            }
                        } else {
                            if a & 1 == 0 {
                                ip = branchip;
                            }
                        }
                    },
                    "jio" => { // Jump if One
                        let branchip:i32 = ip + parts[2].parse::<i32>().unwrap()-1;
                        if var == "b," {
                            if b == 1 {
                                ip = branchip;
                            }
                        } else {
                            if a == 1 {
                                ip = branchip;
                            }
                        }
                    },
                    _ => { assert!(false); }
                }
            }
        }
    }
    b
}

fn process(inp:&String) -> (i32, i32)
{
    let lines:Vec<&str> = inp.lines().collect();
    let part1 = runab(0, &lines);
    let part2 = runab(1, &lines);
    (part1, part2)
}

fn main() {
    let args:Vec<String> = env::args().collect();
    let fname = if args.len() > 1 { args[1].clone() } else { "input.txt".to_string() };
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process(&input); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}