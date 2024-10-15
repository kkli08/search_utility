use std::env;
use std::path::Path;
use glob::glob;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;


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

fn is_file_or_wildcard(arg: &str) -> bool {
    let path = Path::new(arg);
    // check if it is a file or wildcard pattern
    path.extension().is_some() || arg.contains('*')
}

#[allow(dead_code)]
fn debug_print_info(options: &Vec<String>, pattern: &Option<String>, files: &Vec<String>, paths: &Vec<String>) {
    // Display categorized components
    println!("Options: {:?}", options);
    if let Some(p) = pattern {
        println!("Pattern: {}", p);
    } else {
        println!("No pattern provided.");
    }
    println!("Files: {:?}", files);
    println!("Paths: {:?}", paths);
}

fn push_files(arg: &str, files: &mut Vec<String>){
    if arg.contains('*') {
        // wildcard pattern
        match glob(arg) {
            Ok(paths) => {
                for entry in paths {
                    if let Ok(path) = entry {
                        if path.is_file() {
                            if let Some(file_str) = path.to_str() {
                                files.push(file_str.to_string());
                            }
                        }
                    }
                }
            }
            Err(_) => {println!("Invalid argument: {}", arg);}
        }
    } else{
        // single file
        files.push(arg.to_string());
    }
}

fn process_file(file_name: &String, pattern: &Option<String>, grep_option: &GrepOption) -> io::Result<()> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    // For colored output
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    if let Some(ref pattern) = pattern {
        for (line_number, line) in reader.lines().enumerate() {
            let line = line?;
            let mut match_found = if grep_option.case_insensitive {
                line.to_lowercase().contains(&pattern.to_lowercase())
            } else {
                line.contains(pattern)
            };

            if grep_option.invert_match {
                match_found = !match_found;
            }

            if match_found {
                // print filename if flag is true
                if grep_option.print_filename {
                    print!("{}: ", file_name);
                }

                // print line number if flag is true
                if grep_option.print_line_numbers {
                    print!("{}: ", line_number + 1);
                }

                // print the line with color if flag is true AND grep_options.invert_match is false
                if grep_option.colored_output && !grep_option.invert_match {
                    // Find all positions of the matching pattern
                    let mut start = 0;
                    // Match with case-insensitivity or case-sensitivity
                    while let Some(match_index) = if grep_option.case_insensitive {
                        line[start..].to_lowercase().find(&pattern.to_lowercase())
                    } else {
                        line[start..].find(pattern)
                    } {
                        // Print the part before the match
                        print!("{}", &line[start..start + match_index]);

                        // Set the color for the matched word and print it
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                        write!(&mut stdout, "{}", &line[start + match_index..start + match_index + pattern.len()])?;
                        stdout.reset()?;

                        // Update start index to continue searching
                        start += match_index + pattern.len();
                    }

                    // Print the rest of the line after the last match
                    println!("{}", &line[start..]);
                } else {
                    println!("{}", line);
                }
            }
        }
    } else {
        eprintln!("Error: No pattern provided.");
    }

    Ok(())
}


fn main() {
    // collect arguments, skip the program name
    let args: Vec<String> = env::args().skip(1).collect();
    let mut grep_options = GrepOption::new();

    let mut options = Vec::new();
    let mut pattern: Option<String> = None;
    let mut files: Vec<String> = Vec::new();
    let mut paths: Vec<String> = Vec::new();

    for arg in args {
        if arg.starts_with("-") {
            options.push(arg.to_string());
            grep_options.match_arg(arg.as_str());
        } else if pattern.is_none() {
            pattern = Some(arg.to_string());
        } else {
            if is_file_or_wildcard(&arg){
                push_files(&arg, &mut files);
            } else {
                paths.push(arg.to_string());
            }
        }
    }

    // // Debugging info
    // debug_print_info(&options, &pattern, &files, &paths);
    // println!("{:?}", grep_options);

    // check help options
    if grep_options.show_help {
        print_help_info();
        return;
    }

    // search in file
    for file in files {
        if let Err(e) = process_file(&file, &pattern, &grep_options) {
            eprintln!("Error processing file {}: {}", file, e);
        }
    }
    // search in paths


}
