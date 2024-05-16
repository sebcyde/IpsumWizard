fn main() {
    println!("Running ipsum wizard");

    let args: Vec<String> = std::env::args().collect();

    // The first argument (index 0) is the name of the program
    println!("Program name: {}", args[0]);

    // Starting parameters start from index 1
    // Check if any starting parameters are provided
    if args.len() > 1 {
        println!("Starting parameters:");
        // Iterate through starting parameters
        for (index, arg) in args.iter().enumerate().skip(1) {
            println!("{}: {}", index, arg);
        }
    } else {
        println!("No starting parameters provided.");
    }
}
