use clap::App;

fn main() {
    let _matches = App::new("echor")
        .version("0.1.0")
        .author("Hamilton G. Jr. <hamiltonjr2010@gmail.com>")
        .about("Rust echo")
        .get_matches();
}
