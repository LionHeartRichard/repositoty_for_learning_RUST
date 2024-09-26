use num::complex::Complex;

fn main() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {} = {}", a, b, result);

    println!("действительная часть: {} мнимая часть: {}", result.re, result.im); 
}
