fn main() {
    let t = fahrenheit_to_celsius(50.0);
    println!("T = {t}");

    println!("Fibo {}", fibo(0));
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

fn fibo(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    let mut tmp = 0;
    if n == 0 {
        return a;
    }
    if n == 1 {
        return b;
    }

    for _i in 2..n+1 {
        tmp = a + b;
        a = b;
        b = tmp;
    }
    tmp
}