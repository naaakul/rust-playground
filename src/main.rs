fn main() {
    println!("{}", fib(10));
}

fn fib(num: u32) -> u32 {
    let mut fir = 0;
    let mut sec = 1;

    if num == 0 {
        return fir;
    }

    if num == 1 {
        return sec;
    }

    for _ in 0..(num - 1) {
        let temp = sec;
        sec = sec + fir;
        fir = temp;
    }

    return sec;
}
