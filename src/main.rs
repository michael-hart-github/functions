fn main() {
    let my_name = String::from("Johnny");
    greet_user(my_name);
    let sum = calculate_sum(5,10);
    println!("The value of calculate_sum is: {}", sum);
}

/* The function below will take in TWO integer parameters and return ONE value
    1) (a:i32, b:i32) creates two integer parameters named "a" and "b"
    2) -> i32 states that we will return one value. Its name is MUST NOT be specified
    When there is a "->"" in the function, it indicates that the function will RETURN something.
    So, "-> i32" says it will RETURN a value of integer 32 to the main function.
    */
fn calculate_sum(a:i32, b:i32) -> i32 {
    /* We have made variables(?) a and b as integers. We need to add them together to get a value to return to the main function.
        Lines that HAVE a semicolon are called a "statement"
        */
    let sum = a+b; // this is a statement

    /* Note: sum needs to be returned to the main function.
        In returning the variable sum to the main function, it WILL NOT have a semicolon.
        Lines that DO NOT have a semicolon are called an "expression"
        We use expressions to return values from functions
        */
    sum // this is an expression
}

fn greet_user(name:String) {
    println!("{} ...{}! Get in loser! We're programming in Rust!", name,name);
}