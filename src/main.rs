use clap::{Arg, Command};
use std::fs::File;
use std::io::Read;

struct Input {
    string: Option<String>,
    file: Option<String>,
}

struct Output {
    hash: String,
}

fn main() {
    let matches = Command::new("md5")
        .version("1.0.0")
        .author("Philipp Speck <philipp@typo.media>")
        .about("Hashes a string or file using MD5")
        .arg(
            Arg::new("string")
                .short('s')
                .long("string")
                .value_name("STRING")
                .help("String to be hashed")
        )
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("File to be hashed")
        )
        .help_template(
            "{bin} {version} {author}\n\
            {about}\n\n\
            {usage-heading} {usage}\n\n\
            {all-args}{after-help}\n",
        ) // Custom help template to include author
        .get_matches();

    let input = Input {
        string: matches.get_one::<String>("string").cloned(),
        file: matches.get_one::<String>("file").cloned(),
    };

    let mut output = Output {
        hash: String::new(),
    };

    if input.string.is_none() && input.file.is_none() {
        eprintln!("Please provide a string or a file to be hashed");
        std::process::exit(1);
    }

    if let Some(ref data) = input.string {
        let digest = md5::compute(data.as_bytes());
        output.hash = format!("{:x}", digest);
    }

    if let Some(ref file_path) = input.file {
        let mut file = File::open(file_path).expect("Error opening file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Error reading file");
        let digest = md5::compute(&buffer);
        output.hash = format!("{:x}", digest);
    }

    println!("{}", output.hash);
}
