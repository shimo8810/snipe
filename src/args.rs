use clap::Parser;

/// VSCode Snipet Generator
#[derive(Parser, Debug)]
pub struct Args {
    /// prefix of snipet
    pub prefix: String,

    /// input file
    pub file: Option<String>,

    /// description of snipet
    #[clap(short, long, value_parser, default_value = "")]
    pub description: String,
}

impl Args {
    pub fn parse() -> Self {
        Parser::parse()
    }
}
