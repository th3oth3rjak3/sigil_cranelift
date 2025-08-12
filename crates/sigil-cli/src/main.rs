fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: sigil <file.sgl>");
        std::process::exit(1);
    }

    let filename = &args[1];
    let source = std::fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", filename, err);
        std::process::exit(1);
    });

    sigil_core::run(&source);
}
