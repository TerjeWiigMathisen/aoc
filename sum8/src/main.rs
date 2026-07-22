
fn sum8() -> usize
{
    let mut count = 0;
    for i in 1..8 {
        let n = i*100;
        for j in 0..9-i {
            let k = 8-i-j;
            let number = n + j*10 + k;
            println!("{number}, {}", number % 7);
            if number % 7 == 0 {count += 1;}
        }
    }
    count
}

fn main() {
    let count = sum8();
    println!("Total count = {count}");
}
