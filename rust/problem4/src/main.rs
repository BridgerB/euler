

fn main() {
    let mut largest = 0;
    for number in 111..1000 {
        for number2 in 111..1000 {
            let x = number * number2;
            // println!("{x}");
            let s: String = x.to_string();
            if is_palindromic(s) {
                // println!("{x} is palindromic");
                if x > largest {
                    largest = x;
                }
            }
        }
    }
    println!("{largest}")
}

fn is_palindromic(mut foo: String) -> bool {
    let first: String = foo.chars().take(1).collect();
    let last: String = foo.chars().last().unwrap().to_string();
    if first == last {
        // println!("{s}");
        foo.pop();
        foo.remove(0);
        // println!("{s}");
        if foo.len() <=1 {
            return true;
        } else {
            is_palindromic(foo)
        }
    } else {
        return false;
    }
}
