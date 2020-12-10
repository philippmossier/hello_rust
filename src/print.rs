pub fn run(){
    // Print to console
    println!("Hello from print.rs file!");
    // Basic Formatting
    println!("{} is from {}", "Phil", "Earth");
    // Positional Arguments
    println!("{0} is from {1} and likes to {2}", "Phil", "Earth", "code");
    // Named Arguments
    println!("{name} likes to {activity}", name = "Phil" , activity = "run" );
    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10 );
    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));
    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}  