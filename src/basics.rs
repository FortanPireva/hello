

fn multiply(a: i32, b: i32) -> i32 {
    return a * b
}

pub fn fibonacci(n:i32) -> i32 {
    if n<=2 {
        return 1;
    }
    return fibonacci(n-1) + fibonacci(n-2);
}
pub fn collatz_sequence(mut n:i32) -> i32 {
    let mut len = 1;
    while n > 1 {
        n = if n%2 ==0 {n/2} else {3*n+1};
        len += 1;
    }
    len
}
