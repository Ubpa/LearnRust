fn main() {
//  let guess = "42".parse().expect("Not a number");
//      ^^^^^ consider giving `guess` a type
    let guess: u32 = "42".parse().expect("Not a number");
//             ^^^ 帮助 parse() 推断类型
    println!("guess: {}", guess);

    // 1. scalar
    // 1.1 int
    {
        let x : i32 = 4;
        let y : u8 = b'c';
        println!("x: {}, y: {}", x, y);
    }
    // 1.2 float
    {
        let x = 1.0/3.0; // f64
        let y: f32 = 1.0/3.0; // f32
        println!("x: {}, y: {}", x, y);
    }

    // 1.3 char
    {
        let c = 'z';
        let z = 'ℤ';
        let heart_eyed_cat = '😻';
        println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);
    }

    // 2. compound
    // 2.1 tuple
    {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let (x, y, z) = tup;
        println!("x: {}, y: {}, z: {}", x, y, z);
        println!("x: {}, y: {}, z: {}", tup.0, tup.1, tup.2);
    }

    // 2.2 array
    {
        let a = [1, 2, 3, 4, 5];

        let first = a[0];
        let second = a[1];
        println!("first: {}, second: {}", first, second);
    }
}
