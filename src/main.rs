
fn main() {
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