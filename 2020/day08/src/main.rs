// Fastest run (Surface): 8.8 us
//              Acer:       5.2 us
use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;

struct Opcode {
    name: u16,
    delta:i16,
}

fn parse_opcodes(inp:&str) -> Vec<Opcode>
{
    let mut result = Vec::new();
    let bytes = inp.as_bytes();
    let mut i = 0;
    //let mut line = 1;
    while i < bytes.len() {
        let n = bytes[i];
        let name = match n {
            b'n' => 0,
            b'j' => 1,
            b'a' => 2,
            _ => panic!("Unknown opcode"),
        };
        i += 4;
        let mut sign = 1;
        if bytes[i] == b'-' { sign =-1; i += 1 }
        else if bytes[i] == b'+' { i += 1 }
        let mut delta:i16 = (bytes[i] - b'0') as i16; i += 1;
        while bytes[i] != b'\n' {
            delta = delta * 10 + (bytes[i] - b'0') as i16;
            i += 1;
        }
        delta *= sign;
        //if delta == 1 && name == 1 { delta = 0; name = 2;} // NOP acc
        // println!("Parsing line {line}: {} {delta}", match name {
        //     0 => "nop",
        //     1 => "jmp",
        //     2 => "acc",
        //     _ => panic!("Unknown opcode"),
        // });
        //line += 1;
        result.push(Opcode { name: name, delta: delta });
        i += 1;
    }
    result
}

fn run(program:&[Opcode], visited:&mut Vec<bool>) -> (i16, bool)
{
    let mut acc = 0;
    let mut ip:i16 = 0;
    *visited = vec![false; program.len()];
    while (ip as usize) < program.len() {
        if visited[ip as usize] { return (acc, true); }
        visited[ip as usize] = true;
        let opcode = &program[ip as usize];
        match opcode.name {
            0 => ip += 1,
            1 => ip += opcode.delta,
            2 => { acc += opcode.delta; ip += 1; },
            _ => panic!("Unknown opcode"),
        }
    }
    (acc, false)
}

fn process(inp:&str) -> (i16, i16)
{
    let mut program = parse_opcodes(inp);
    let mut visited = Vec::new();
    let (part1, _inf) = run(&program, &mut visited);
    //println!("Part1 = {part1}, visited = {}", visited.iter().filter(|&&v| v).count());
    //let mut candidates = Vec::new();
    for i in (0..program.len()).rev() {
        if !visited[i] || program[i].name >= 2 { continue; }
//        if program[i].name == 1 && program[i].delta == 1 { continue}
//        println!("Trying to swap line {} {}",i+1, if program[i].name == 0 { "nop" } else { "jmp" });
        let original = program[i].name;
        program[i].name = original ^ 1; // swap n<->j
        let (part2, inf) = run(&program, &mut visited);
        if !inf { return (part1, part2); }
        program[i].name = original; // restore
    }
    (0,0)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.len()-1] != b'\n' {input.push('\n');}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process(&input); }); bench_result.print_stats();

    //process(&input);
    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us for 1000 runs",devtime.time_in_micros().unwrap());
}