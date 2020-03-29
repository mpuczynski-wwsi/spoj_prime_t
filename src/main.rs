use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let n:u32 = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..n{
        let m:u16 = lines.next().unwrap().unwrap().parse().unwrap();
        if is_prime(m) {
            println!("TAK")
        } else {
            println!("NIE")
        }
    }
}

fn is_prime(m: u16) -> bool{
    if m < 2 {
        return false;
    } else {
        for i in 2..m {
            if m % i == 0 {
                return false;
            }
        }
        true
    }
}

