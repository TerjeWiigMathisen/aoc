// Fastest run: Surface Pro 8, 10.3 us
//              Acer 3.0 us

//use devtimer::DevTime;
use devtimer::run_benchmark;
use std::arch::x86_64::*;

fn process(inp:&String)->(usize, usize)
{
    //                       AX BX CX   AY BY CY   AZ BZ CZ
    let part1tab:[u8;32] = [0,4,1,7,0,8,5,2,0,3,9,6,0,0,0,0,0,4,1,7,0,8,5,2,0,3,9,6,0,0,0,0];
    let part2tab:[u8;32] = [0,3,1,2,0,4,5,6,0,8,9,7,0,0,0,0,0,3,1,2,0,4,5,6,0,8,9,7,0,0,0,0];
    let input = inp.as_bytes();
    let blocks = input.len()/64;
    unsafe {
        let mut part1 = _mm256_setzero_si256();
        let mut part2 = _mm256_setzero_si256();
        let abc_mask = _mm256_set1_epi32(0x00000003);
        let xyz_mask = _mm256_set1_epi32(0x00030000);
        let part1shuffle = _mm256_loadu_si256(part1tab.as_ptr() as *const __m256i);
        let part2shuffle = _mm256_loadu_si256(part2tab.as_ptr() as *const __m256i);
        for b in 0..blocks {
            let bl = input.as_ptr().add(b*64) as *const __m256i;
            let b1 = _mm256_loadu_si256(bl);
            let b2 = _mm256_loadu_si256(bl.add(1));
            let b1h = _mm256_and_si256(b1, xyz_mask);
            let b2h = _mm256_and_si256(b2, xyz_mask);
            let b1l = _mm256_and_si256(b1, abc_mask);
            let b2l = _mm256_and_si256(b2, abc_mask);
            let b1h = _mm256_srli_epi32(b1h, 14);
            let b2h = _mm256_srli_epi32(b2h, 14);
            let b1hash = _mm256_or_si256(b1l, b1h);
            let b2hash = _mm256_or_si256(b2l, b2h);
            let b16 =_mm256_packus_epi32(b1hash, b2hash);
            let inc1 = _mm256_shuffle_epi8(part1shuffle, b16);
            let inc2 = _mm256_shuffle_epi8(part2shuffle, b16);
            part1 = _mm256_add_epi16(part1, inc1);
            part2 = _mm256_add_epi16(part2, inc2);
        }
        let mut part1mem:[u16;16] = [0;16];
        _mm256_storeu_si256(part1mem.as_mut_ptr() as *mut __m256i, part1);
        let mut p1 = 0;
        let mut part2mem:[u16;16] = [0;16];
        _mm256_storeu_si256(part2mem.as_mut_ptr() as *mut __m256i, part2);
        let mut p2 = 0;
        for i in 0..16 {
            p1 += part1mem[i] as usize;
            p2 += part2mem[i] as usize;
        }
//        println!("part1mem={:?}", part1mem);
//        println!("part2mem={:?}", part2mem);
        return (p1, p2);
    }
}

fn main() {
    let mut input = std::fs::read_to_string("input.txt").unwrap();
//    let mut input = std::fs::read_to_string("test.txt").unwrap();
    while input.as_bytes().len() % 64 != 0 {
        input.push(' ');
    }

    let bench_result = run_benchmark(1000, |_| {
        process(&input);
    });
    bench_result.print_stats();

    let (part1, part2) = process(&input);
    println!("part1={part1}");
    println!("part2={part2}");
}
