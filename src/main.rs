use clap::{App, Arg};

fn main() {
    // flags handling using clap
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Hamilton G. Jr. <hamiltonjr2010@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
    .get_matches();

    // text handling
    let text = matches.values_of_lossy("text").unwrap();
    
    // newline handling
    let omit_newline = matches.is_present("omit_newline");
    
    // output
    print!("{}{}", text.join(" "), 
        if omit_newline { "" } else { "\n" });

}
