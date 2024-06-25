use std::{ env, fs };

fn main() {
    let args: Vec<String> = env::args().collect(); //In collections we need to specify the expected type

    let query = &args[1]; // The first index is the binary path
    let file_name = &args[2];

    let content = fs::read_to_string(file_name).expect(&format!("File not found: {}", file_name));

    println!("The content of the file is: {}", content);
}
