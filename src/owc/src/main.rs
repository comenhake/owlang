use clap::Parser;

/// owc - OwLang compiler CLI
#[derive(Parser, Debug)]
struct Args {
    /// Source file to compile
    #[arg(long)]
    file: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("owc invoked with {:?}", args.file);

    if let Some(file) = args.file {
        // 0. Load source
        let source = std::fs::read_to_string(&file)
            .map_err(|e| anyhow::anyhow!("Failed to read {file}: {e}"))?;

        // 1. Lexer
        let tokens = lexer::tokenize(&source)?;
        println!("Lexed {} tokens", tokens.len());

        // 2. Parser
        let ast = parser::parse(tokens)?;
        println!("Parsed AST: {:?}", ast);

        // 3. Semantic analysis
        semantic::analyze(&ast)?;

        // 4. Codegen
        codegen::generate(&ast)?;
        println!("Compilation successful!");
    } else {
        eprintln!("No input file provided. Use --file <path>");
    }

    Ok(())
}
