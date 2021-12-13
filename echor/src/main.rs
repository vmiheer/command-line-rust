use clap::{App, Arg};
fn main() {
    let args = App::new("echoer")
        .version("0.1.0")
        .author("Miheer Vaidya")
        .about("Rust echo")
        .arg(Arg::with_name("omit_newline").short("n").help("Do not print newline").takes_value(false))
        .arg(Arg::with_name("text").value_name("TEXT").required(true).min_values(1))
        .get_matches();
    // let args = std::env::args().skip(1).peekable();
    let text = args.values_of_lossy("text").unwrap();
    print!("{}", text.join(" "));
    if ! args.is_present("omit_newline") {
        println!("");
    }
}
