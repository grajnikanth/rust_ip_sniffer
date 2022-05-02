// use standard env library to read command line arguments
use std::env;


fn main() {

    // Store the command line input into a vector using args() and collect()
    // function
    let args: Vec<String> = env::args().collect();

    // Initial test printout the args to see what it stored

    for arg in &args {
        println!("{}", arg);
    }

    // Print the vector
    println!("{:?}", args);

}
