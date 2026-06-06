//use std::io;
//use std::env;
use std::fs;
use substring::Substring;
use std::collections::HashMap;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;

#[derive(Clone)]
struct Particle {
    x:i64,y:i64,z:i64, 
    vx:i64,vy:i64,vz:i64,
    ax:i64,ay:i64,az:i64,
}
fn process(inp:String) -> (i32,i32)
{
    let mut p1 = 0;
    let mut min_acc = i64::MAX;
    let mut p2:i32 = 0;
    let input = inp.clone();
    let lines:Vec<&str> = input.split("\n").collect();

    let mut particles:Vec<Particle> = vec![];

    let mut lnr = 0;
    for li in lines {
//        println!("li={li}");
        let parts:Vec<&str> = li.split(", ").collect();
        let pstr = parts[0].substring(3,parts[0].len()-1);
        let pos:Vec<i64> = pstr.split(",").map(|x| x.trim().parse::<i64>().unwrap()).collect();

        let vstr = parts[1].substring(3,parts[1].len()-1);
        let vel:Vec<i64> = vstr.split(",").map(|x| x.trim().parse::<i64>().unwrap()).collect();

        let astr = parts[2].substring(3,parts[2].len()-1);
        let acc:Vec<i64> = astr.split(",").map(|x| x.trim().parse::<i64>().unwrap()).collect();

        let p = Particle{x:pos[0],y:pos[1],z:pos[2],
                vx:vel[0],vy:vel[1],vz:vel[2],
                ax:acc[0],ay:acc[1],az:acc[2]};
        particles.push(p);
    
        let a = acc[0]*acc[0] + acc[1]*acc[1] + acc[2]*acc[2];
        if a < min_acc {
            min_acc = a;
            p1 = lnr;
        }
        lnr += 1;
    }
    lnr = 0;
    let mut stable = 0;
    loop {
        let mut positions = HashMap::new();
        let plen = particles.len();
        let mut collide:Vec<bool> = vec![false;plen];
        for pnr in 0..plen {
            let p:&mut Particle = &mut particles[pnr];
            p.vx += p.ax; p.vy += p.ay; p.vz += p.az;
            p.x += p.vx; p.y += p.vy; p.z += p.vz;
            let key = format!("{},{},{}", p.x,p.y,p.z);
            if positions.contains_key(&key) { // Remove both if collision
                collide[pnr] = true;
                collide[positions[&key]] = true;
            }
            positions.insert(key,pnr); // Last one saved for now if duplicate
        }
        let psave:Vec<Particle> = particles.clone();
        particles.clear();
        for pnr in 0..plen {
            if !collide[pnr] {
                particles.push(psave[pnr].clone());
            }
        }
        lnr += 1;
        if particles.len() == p2 as usize {
            stable += 1;
            if stable > 50 { break; }
        }
        else {
            p2 = particles.len() as i32;
            stable = 0;
        }
//        println!("Iteration {lnr}: {} particles remain", particles.len());
//        if lnr > 1000 {break; }
    }
    return (p1,p2);
}

fn main() {

/*     assert!(process(
"p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>".to_owned()) == (0,0));
 */    let fname = "input.txt"; // instead of args[1]
    let input = fs::read_to_string(fname).expect("Error readin input file");
    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10, |_| {
        process(input.clone());
    });
    bench_result.print_stats();

    devtime.start();
    let (part1,part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}