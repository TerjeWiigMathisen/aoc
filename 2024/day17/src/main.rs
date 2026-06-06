// 9.7 us with full emulation of 1K lookup table
// 11.6 us including parsing of input

use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;

fn _simulate(rega:usize) -> Vec<u8>
{
    let mut ret = Vec::<u8>::new();
    let mut a = rega;
    loop {
        let b = (a & 7) ^ 2;
        let c = a >> b;
        let b = (b ^ 3) ^ c;

        ret.push((b & 7) as u8);
        a >>= 3;
        if a == 0 {break;}
    }
    ret
}

fn _emulate(rega:usize, prog:[u8;16]) -> Vec<u8>
{
    let mut ret = Vec::<u8>::new();
    let mut regs:[usize;7] = [0,1,2,3,rega,0,0];
    let mut ip = 0;
    loop {
        let opc = prog[ip];
        let oper = prog[ip+1] as usize;
        ip += 2;
        assert!(opc <= 7);
        match opc {
            0=>{regs[4] >>= regs[oper];},
            1=>{regs[5] ^= oper;},
            2=>{regs[5] = regs[oper] & 7;},
            3=>{if regs[4] != 0 {ip = oper;}},
            4=>{regs[5] ^= regs[6];},
            5=>{ret.push((regs[oper] as u8) & 7);},
            6=>{regs[5] = regs[4] >> regs[oper];},
            7=>{regs[6] = regs[4] >> regs[oper];},
            _ => {},
        }
        if ip >= prog.len() {return ret;}
    }
}

// Run the emulation only until the first out operation
fn emulate1(rega:usize, prog:[u8;16]) -> u8
{
    // regs[0..=3] == constants, 4,5,6 == rega,regb,regc
    let mut regs:[usize;7] = [0,1,2,3,rega,0,0];
    let mut ip = 0;
    loop {
        let opc = prog[ip];
        let oper = prog[ip+1] as usize;
        ip += 2;
        assert!(opc <= 7);
        match opc {
            0=>{regs[4] >>= regs[oper];},
            1=>{regs[5] ^= oper;},
            2=>{regs[5] = regs[oper] & 7;},
            3=>{if regs[4] != 0 {ip = oper;}},
            //3=>{return ret[0];},
            4=>{regs[5] ^= regs[6];},
            5=>{return (regs[oper] as u8) & 7;},
            6=>{regs[5] = regs[4] >> regs[oper];},
            7=>{regs[6] = regs[4] >> regs[oper];},
            _ => {},
        }
        if ip >= prog.len() {break;}
    }
    0
}

fn recursive_search(prog:[u8;16], lookup:[u8;1024], dig:usize, rega:usize) -> usize
{
    if dig == 16 {return rega;}

    let d3 = dig*3;
    let mut b = rega >> d3;
    let mut mina = usize::MAX;
    let p = prog[dig];
    while b < 1024 {
        if lookup[b] == p {
            let res = recursive_search(prog, lookup, dig+1, rega | (b << d3));
            if res < mina { mina = res;}
        }
        b += 128;
    }
    mina
}

fn search(prog:[u8;16], lookup:[u8;1024]) -> usize
{
    let mut mina = usize::MAX;
    let p0 = prog[0];
    for b in 0..1024 {
        if lookup[b] == p0 {
            let res = recursive_search(prog, lookup, 1, b);
            if res < mina { mina = res;}
        }
    }
    mina
}

fn searchtop(prog:[u8;16], lookup:[u8;1024], dig:usize, rega:usize) -> usize
{
    let r3 = rega << 3;
    let top7 = r3 & 0x3f8;
    let p = prog[dig];
    for b in top7..top7+8 {
        if lookup[b] == p {
            let r = r3 | b;
            if dig == 0 {return r;}
            let res = searchtop(prog, lookup, dig-1, r);
            if res < usize::MAX { return res;}
        }
    }
    usize::MAX
}

fn process(inp:String) -> (String, usize)
{
    let byt = inp.as_bytes();
    let mut p = 0;
    while byt[p] != b':' {p+=1;}
    p += 2;
    let mut rega:usize = 0;
    while byt[p] >= b'0' {
        rega = rega*10 + (byt[p] - b'0') as usize;
        p += 1;
    }
    while byt[p] != b'\n' || byt[p-1] != b'\n' { p += 1;}
    while byt[p-1] != b' ' {p+=1;}
    let mut program:[u8;16] = [0;16];
    for i in 0..16 {
        program[i] = byt[p+i*2] - b'0';
    }
    
    let mut lookup:[u8;1024] = [0;1024];
    for i in 0..1024 {
        let d = emulate1(i, program);
        lookup[i] = d;
    }

    //let p1 = emulate(rega, program);
    let mut p1 = Vec::<u8>::with_capacity(16);
    while rega != 0 {
        p1.push(lookup[rega & 1023]);
        rega >>= 3;
    }
    let part1 = format!("{:?}", p1).replace(" ","");

    //let part2 = searchtop(program, lookup,15,0);
    let part2 = search(program, lookup);
    (part1, part2)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}