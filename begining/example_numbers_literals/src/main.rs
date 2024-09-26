fn main() {

    let twenty = 20;
    let twenty_one = 21;
    let twenty_two = 22_i32;

    let addition = twenty + twenty_one + twenty_two;


    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    let one_million: i64 = 1_000_000;
    println!("{}", one_million);

    let forty_twos = [
        42.0,
        42_f32,
        42.0_f32,
        42f32,
    ];

    println!("{:02}", forty_twos[0]);


    let three = 0b_11;
    let thirty = 0o_36;
    let three_hundred = 0x_12C;

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8: {:o}, {:o}, {:o}", three, thirty, three_hundred);
    println!("base 16: {:x}, {:x}, {:x}", three, thirty, three_hundred);
}
