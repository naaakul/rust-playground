fn main() {
    let n = 10; 
    let mut a = 0;
    let mut b = 1;

    println!("Fibonacci series up to {} terms:", n);

    for _ in 0..n {
        print!("{} ", a);
        let temp = a + b;
        a = b;
        b = temp;
    }
    println!();
}
