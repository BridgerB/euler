fn main() {

    use std::time::Instant;
    let now = Instant::now();
    
    let mut sum = 0;
    let mut largest = 0;
    for i in 1..1000000 {
        sum += i; 
        // println!("{sum}")
        let number1 = divisor_count(sum);
        if number1 > largest {
            largest = number1; 
            println!("{largest}, {i}, {sum}****************************************")
        }
        let elapsed = now.elapsed(); 
        println!("{largest}, {i}, {sum}, {:.2?}, {number1}", elapsed);
    }
}

fn divisor_count(foo: u32) -> u32 {
    let mut count = 1;
    let half_foo = foo/2;
    let third_foo = foo/3;

    if foo % 2 == 0 {
        for j in 1..half_foo+1 {
            if foo % j == 0 {
                count += 1;
            } 

        }
        return count;
    } else if foo % 3 == 0 {
        for j in 1..third_foo+1 {
            if foo % j == 0 {
                count += 1;
            } 
        }
        return count;
    } else {
        return count;
    }
    
}
