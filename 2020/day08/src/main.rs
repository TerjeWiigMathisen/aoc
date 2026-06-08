// Fastest run (Surface): 60.7 us
//              Acer:
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
    for line in inp.lines() {
        let bytes = line.as_bytes();
        if bytes.len() < 5 { continue; }
        let n = bytes[0];
        let name = match n {
            b'n' => 0,
            b'j' => 1,
            b'a' => 2,
            _ => continue,
        };
        result.push(Opcode { name: name, delta: (line[4..line.len()].to_string().parse::<i16>().unwrap_or(0)) });
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
    for i in 0..program.len() {
        if !visited[i] || program[i].name >= 2 { continue; }
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
    let input = fs::read_to_string(fname).expect("Error readin input file");
//    if input.as_bytes()[input.len()-1] != b'\n' {input.push('\n');}

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