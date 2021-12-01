fn main() {
    // default const
    {
        let x = 5;
        println!("The value of x is: {}", x);
    //  x = 6;
    //  ^^^^^ cannot assign twice to immutable variable
        println!("The value of x is: {}", x);
    }

    // mut
    {
        let mut x = 5;
    //      ^^^
        println!("The value of x is: {}", x);
        x = 6;
        println!("The value of x is: {}", x);
    }

    // shadowing
    {
        let x = 5;
        let x = x + 1; // <-------------------------
    //      ^ redefinition / shadowing            //
        {                                         //
            let x = x * 2; // <----------------   //
        //      ^ shadowing again             |   //
            println!("The value of x is: {}", x); //
        }                                         //
                                                  //
        println!("The value of x is: {}", x); //----
    }
    
    // shadowing with different types
    {
        let spaces = "   ";
    //  spaces = spaces.len();
    //         ^ error type => can't assign
        let spaces = spaces.len();
    //      ^^^^^^ different types: string -> integer
        println!("The length of spaces is: {}", spaces);
    }
}
