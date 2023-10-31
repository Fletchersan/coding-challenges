use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect::<Vec<_>>()[1..].to_vec();
    // args = args[1..].to_vec();
    // println!("{:?}", args);
    // arbitrary number of args
    // to be split into -flags and files
    let mut flags: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();
    for arg in args.iter(){
        if arg.chars().nth(0) == Some('-') {
            for char in arg.chars() {
                if char == '-' {
                    continue;
                } else {
                    
                    flags.push(String::from(char));
                }
            }
        } else {
            files.push(String::from(arg));
        }
    }
    // println!("{:?}", flags);
    // println!("{:?}", files);
    let file_path = &files[0];


    // println!("With text: \n{contents}")
    let mut values: Vec<usize> = Vec::new();
    let contents = fs::read_to_string(file_path).expect("Should be able to read file.");

    for flag in flags.iter() {
        if *flag == String::from('c') || *flag == String::from('m') {
            // count the number of bytes in the text
            let num_bytes:usize = contents.as_bytes().len();
            // println!("num bytes = {num_bytes}");
            values.push(num_bytes);
        } else if *flag == String::from('l') {
            // count number of lines in the file
            let lines = contents.lines();
            let mut line_num: usize = 0;
            for _line in lines {
                line_num += 1;
            }
            // println!("{line_num}");
            // println!("Number of lines {line_num}");
            values.push(line_num)
        } else if *flag == String::from('w') {
            let mut words: usize = 0;

            let mut in_word: bool = true;

            for ch in contents.chars() {
                if !ch.is_whitespace() {
                    in_word = true;
                } else if in_word {
                    words += 1;
                    in_word = false;
                }
            }
            values.push(words);
        }

    }
    for value in values{
        print!("{}\t", value);
    }
    println!("{}", file_path);
}
