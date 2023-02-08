use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("searching for file {}", file_path);

    let contents = fs::read(file_path)
        .expect("Should have been able to read the file");
    
    let mut result = String::from("");
    
    for (i, byte) in contents.iter().enumerate(){
        if i % 16 == 0 && i != 0 {
            result.push('\n')
        }
        result.push_str(&format!("{:x} ", byte));
    }
    println!("{}", result)
}

fn disassemble_8080(buffer: &String, pc: usize){

}
