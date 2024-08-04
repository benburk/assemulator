use pest::{iterators::Pairs, Parser};

/// Pest parser for the assembler.
#[derive(pest_derive::Parser)]
#[grammar = "./src/assembler/grammar.pest"]
struct AsmParser;

pub fn parse(text: &str) -> Result<Pairs<Rule>, String> {
    AsmParser::parse(Rule::lines, text).map_err(|e| e.to_string())
}
