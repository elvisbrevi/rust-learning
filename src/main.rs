
fn main() {
    
    //prints();

    variables();

}

fn variables() {
    // -----------------------------
    //  Variables in Rust
    // -----------------------------
    let mut x: i64 = 10;
    println!("The value of x is {}", x);
    x = 20;

    // -----------------------------
    //  - Data Types
    //      - Scalar Data Types
    //          - Integer
    //              - Signed i8, i16, i32, i64, i128, isize  
    //                  - 2^(i-1) -1 to 2^(i-1) -1
    //                  - 2^(8-1) -1 to 2^(8-1) -1 = -127 to 127
    //              - Unsigned u8, u16, u32, u64, u128, usize           
    // -----------------------------
    println!("the maximun value of i8 is {}", std::i8::MAX);
}

#[allow(dead_code)]
fn prints() {
    // Print a string
    println!("The number is {}", 10);
    // Print a string with multiple arguments
    println!("The number is {}, and the letters are {} and {}", 1, 'a', "b");
    // Print with positional arguments
    println!("The number is {2}, and the letters are {1} and {0}", 'a', "b", 2);
    // Print named arguments
    println!("The number is {number}, and the letters are {letter1} and {letter2}", number=2, letter1='a', letter2="b");
    // Print basic math
    println!("The number is {}", 10 + 10);
    // Print multiple lines
    print!("Multiple
            line");
    // Print a new line
    println!("\n\n Print after two lines");
    // Print a tab
    println!("Print a tab \t now");
    // Print a backspace and others special characters
    println!("Print a backslash \\ or a double backslash \\\\ or comma \" or a single quote \'");
    // Print a overwriten text
    println!("This text wich will be overwriten \r new text ");
}