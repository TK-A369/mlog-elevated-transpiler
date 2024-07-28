mod parser;
mod tokenizer;

use clap;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    input_file: String,
}

fn main() {
    let args = Args::parse();
    println!("Transpiling {}...", args.input_file);

    let file_contents = std::fs::read_to_string(&args.input_file)
        .expect(format!("Error occurred during reading file {}", &args.input_file).as_str());

    let tokens = tokenizer::tokenize(file_contents.as_str()).unwrap();

    println!("Tokens:\n{:?}", tokens);

    let ast = parser::parse_program(&tokens).unwrap();

    println!("AST:\n{:?}", ast);
}
