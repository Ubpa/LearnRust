fn main() {
    // basic if
    {
        let number = 6;

        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4, 3, or 2");
        }
    }

    // if expression
    {
        let condition = true;
        // all branches must be expressions
        // all values of expressions must be same type
        let number = if condition {
            5 /* no ; */
        } else {
            6 /* no ; */
        };

        println!("number: {}", number);
    }

    // label loop
    {
        'outer: loop {
            loop {
                break 'outer;
            }
            println!("no entry!"); // unreachable statement
        }
    }

    // loop break expression
    {
        let x = loop {
            break 1;
        };
        println!("x: {}", x);
    }

    // while
    {
        let mut number = 3;

        while number != 0 {
            println!("{}!", number);
    
            number = number - 1;
        }
    
        println!("LIFTOFF!!!");
    }

    {
        let a = [10, 20, 30, 40, 50];

        for element in a.iter() {
            println!("the value is: {}", element);
        }
    }

    {
        for number in (1..4).rev() {
            println!("{}!", number);
        }
        println!("LIFTOFF!!!");
    }

    // loop
    {
        loop {
            println!("Press `Ctrl + C` to stop!");
        }
    }
}
