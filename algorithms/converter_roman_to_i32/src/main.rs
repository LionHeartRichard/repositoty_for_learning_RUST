fn main() {
    
}


pub fn roman_to_int(s: char) -> i32 {
    let mut num = 0;
    if s.len == 1 {
        return converter(s[0]);
    }
    for item in .................... {
        let first = converter(item);
        num+=first;
    }
    num
}

fn converter(num: char) -> i32 {
    match num {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
    }
    -1
}
