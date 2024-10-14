use std::env;

#[derive(Debug)]
struct GrepOption {
    case_insensitive: bool, // -i Case-insensitive search
    print_line_numbers: bool, // -n Print line numbers
    invert_match: bool, // -v Invert match (exclude lines that match the pattern)
    recursive_search: bool, // -r Recursive directory search
    print_filename: bool, // -f Print filenames
    colored_output: bool, // -c Enabled colored output
    show_help: bool, // -h, --help Show help information
}

impl GrepOption {
    fn new() -> Self {
        GrepOption {
            case_insensitive: false,
            print_line_numbers: false,
            invert_match: false,
            recursive_search: false,
            print_filename: false,
            colored_output: false,
            show_help: false,
        }
    }

    fn match_arg(&mut self, arg: &str) {
        match arg {
            "-i" => self.case_insensitive = true,
            "-n" => self.print_line_numbers = true,
            "-v" => self.invert_match = true,
            "-r" => self.recursive_search = true,
            "-f" => self.print_filename = true,
            "-c" => self.colored_output = true,
            "-h" | "--help" => self.show_help = true,
            _ => {},
        }
    }
}



fn print_help_info() {
    println!("Usage: grep [OPTIONS] <pattern> <files...>");
    println!("Options:");
    println!("-i                Case-insensitive search");
    println!("-n                Print line numbers");
    println!("-v                Invert match (exclude lines that match the pattern)");
    println!("-r                Recursive directory search");
    println!("-f                Print filenames");
    println!("-c                Enable colored output");
    println!("-h, --help        Show help information");
}


fn main() {
    // collect arguments, skip the program name
    let args: Vec<String> = env::args().skip(1).collect();
    let mut grep_options = GrepOption::new();

    let mut options = Vec::new();
    let mut pattern: Option<String> = None;
    let mut files: Vec<String> = Vec::new();

    for arg in args {
        if arg.starts_with("-") {
            options.push(arg.to_string());
            grep_options.match_arg(arg.as_str());
        } else if pattern.is_none() {
            pattern = Some(arg.to_string());
        } else {
            files.push(arg.to_string());
        }
    }

    // Display categorized components
    println!("Options: {:?}", options);
    if let Some(p) = pattern {
        println!("Pattern: {}", p);
    } else {
        println!("No pattern provided.");
    }
    println!("Files: {:?}", files);


    // // Debugging info
    // println!("{:?}", grep_options);

    // check help options
    if grep_options.show_help {
        print_help_info();
        return;
    }
}
