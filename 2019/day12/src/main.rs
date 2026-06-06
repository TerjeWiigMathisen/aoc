//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

#[derive(Clone,Debug)]
struct Point3 {
    x:i32,
    y:i32,
    z:i32,
    vx:i32,
    vy:i32,
    vz:i32,
}
impl Point3 {
    fn energy(&self)->i32
    {
        let pot = self.x.abs()+self.y.abs()+self.z.abs();
        let kin = self.vx.abs()+self.vy.abs()+self.vz.abs();
        pot * kin
    }
    fn _dump(&self)
    {
        println!("pos=<x={:2}, y={:2}, z={:2}>, vel=<x={:2}, y={:2}, z={:2}>", self.x,self.y,self.z,self.vx,self.vy,self.vz);
    }
}

fn _dump(moons:&[Point3], head:String)
{
    println!("{head}");
    for m in moons.iter().take(4) {
        m.dump();
    }
    println!();
}

fn process(inp:&str, generations:usize) -> i64
{
    let p = parser!(lines("<x=" x:i32 ", y=" y:i32 ", z=" z:i32 ">" => Point3{x,y,z, vx:0, vy:0, vz:0}));
    let mut moons = p.parse(&inp).unwrap();
//    dump(&moons,"After 0 steps:".to_owned());
    for _gen in 1..=generations {
        for m0 in 0..3 {
            for m1 in m0+1..4 {
                let sx = (moons[m0].x - moons[m1].x).signum();
                moons[m0].vx -= sx; moons[m1].vx += sx;
                let sy = (moons[m0].y - moons[m1].y).signum();
                moons[m0].vy -= sy; moons[m1].vy += sy;
                let sz = (moons[m0].z - moons[m1].z).signum();
                moons[m0].vz -= sz; moons[m1].vz += sz;
            }
        }
        for m in moons.iter_mut().take(4) {
            m.x += m.vx;
            m.y += m.vy;
            m.z += m.vz;
        }
        //dump(&moons, format!("After {_gen} steps:"));
    }
//    dump(&moons, format!("After {generations} steps:"));
    let pot = moons[0].energy()+moons[1].energy()+moons[2].energy()+moons[3].energy();
    pot as i64
}

fn process2(inp:&str, generations:usize) -> i64
{
    let p = parser!(lines("<x=" x:i32 ", y=" y:i32 ", z=" z:i32 ">" => Point3{x,y,z, vx:0, vy:0, vz:0}));
    let mut moons = p.parse(&inp).unwrap();
//    dump(&moons,"After 0 steps:".to_owned());
    let omoons:[Point3;4] = [moons[0].clone(), moons[1].clone(), moons[2].clone(), moons[3].clone()];
    let mut xyz = 0;
    let mut px:i64 = 0;
    let mut py:i64 = 0;
    let mut pz:i64 = 0;
    for _gen in 1..=generations {
        for m0 in 0..3 {
            for m1 in m0+1..4 {
                let sx = (moons[m0].x - moons[m1].x).signum();
                moons[m0].vx -= sx; moons[m1].vx += sx;
                let sy = (moons[m0].y - moons[m1].y).signum();
                moons[m0].vy -= sy; moons[m1].vy += sy;
                let sz = (moons[m0].z - moons[m1].z).signum();
                moons[m0].vz -= sz; moons[m1].vz += sz;
            }
        }
        for m in moons.iter_mut().take(4) {
            m.x += m.vx;
            m.y += m.vy;
            m.z += m.vz;
        }
        if moons[0].x == omoons[0].x && moons[1].x == omoons[1].x && moons[2].x == omoons[2].x  && moons[3].x == omoons[3].x && moons[0].vx == 0 {
//            dump(&moons, format!("X match after {_gen} steps:"));
            xyz |= 1;
            if px == 0 {px = _gen as i64;}

        }
        if moons[0].y == omoons[0].y && moons[1].y == omoons[1].y && moons[2].y == omoons[2].y  && moons[3].y == omoons[3].y && moons[0].vy == 0 {
//            dump(&moons, format!("Y match after {_gen} steps:"));
            xyz |= 2;
            if py == 0 {py = _gen as i64;}
        }
        if moons[0].z == omoons[0].z && moons[1].z == omoons[1].z && moons[2].z == omoons[2].z  && moons[3].z == omoons[3].z && moons[0].vz == 0 {
//            dump(&moons, format!("Z match after {_gen} steps:"));
            xyz |= 4;
            if pz == 0 {pz = _gen as i64;}
        }
        if xyz == 7 {break;}
    }
    let mut pot = px*py*pz;
    for p in [2,3,5,7,11,13,17,19].iter() {
        while pot % *p == 0 {
            let pd = pot / *p;
            if pd % px != 0 || pd % py != 0 || pd % pz != 0 {break}
            pot = pd;
        }
    }
    println!("Back at starting configuration after {pot} generations");
    pot
}

fn main() {
    assert!(process("<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>",10) == 179);
assert!(process(
"<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>",100) == 1940);
assert!(process2(
"<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>",10000) == 4686774924);
    //    assert!(process("3,3,1107,-1,8,3,4,3,99",vec![7]) == 1);

//    let bench_result = run_benchmark(10, |_| {bench_permute();}); bench_result.print_stats();

    //panic!("Stop now");

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
//    if input.as_bytes()[input.as_bytes().len()-1] == b'\n' {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10, |_| { 
        process(&input,1000);
        process2(&input, 1000000);
     }); bench_result.print_stats();

    devtime.start();
    let part1 = process(&input,1000);
    let part2 = process2(&input, 1000000);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}