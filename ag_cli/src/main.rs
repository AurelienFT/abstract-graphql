use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Usage: {} [your_graphql_file]", args[0]);
        return;
    }

    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    println!("{}", contents);
    let tokens = ag_parsing::tokenize(contents).unwrap();
    ag_codegen::typescript::write_typescript(tokens).unwrap();
}
