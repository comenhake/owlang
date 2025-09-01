use clap::Parser;

/// owc - OwLang compiler CLI
#[derive(Parser)]
struct Args {
    #[arg(long)]
    file: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("owc invoked with {:?}", args.file);

    if let Some(file) = args.file {
        let source = std::fs::read_to_string(&file)?;

        // 1. Lexer
        let tokens = lexer::tokenize(&source)?;

        // 2. Parser
        let ast = parser::parse(tokens)?;

        // 3. Semantic analysis
        //semantic::analyze(&ast)?;

        // 4. Codegen
        //let output = codegen::generate(&ast)?;
        //println!("Compilation successful!\n{output}");
    }

    Ok(())
}
