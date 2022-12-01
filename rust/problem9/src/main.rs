
use std::process;

fn main() {
    for a in 1..1000 {
        for b in 1..1000 {
            let a2: f64 = a as f64 * a as f64;
            let b2: f64 = b as f64 * b as f64;
            let z: f64 = a2+b2;
            let c = f64::sqrt(z);
            // println!("{c}");
            let sum = a as f64 + b as f64 + c as f64;
            // println!("{sum}")
            if sum == 1000 as f64 {
                // println!("{a} {b} {c}");
                let answer = a as f64 * b as f64 *c;
                println!("{answer}");
                process::exit(1);
                
            }
        }
    }
}
