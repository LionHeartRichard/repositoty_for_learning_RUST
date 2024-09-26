fn main() {
    for (x, y) in (0..).zip(0..) {
        if x+y > 100 {
            break;
        }
         println!("x={:?}, y={:?}, x+y={:?}",x,y, (x+y));
    }

    println!("-----------------------------------------------------------------------");

    'outer: for x in 0.. {
        'medium: for y in 0.. {
                    for z in 0.. {
                        if x+y+z > 100 {
                            break 'outer;
                        }
                        if z>50 {
                            break;
                        }
                        if y > 30 {
                            break 'medium;
                        }
                        println!("x={:?}, y={:?}, z={:?}, x+y+z={:?}", x, y, z, (x+y+z));
                    }
                }
    }
}
