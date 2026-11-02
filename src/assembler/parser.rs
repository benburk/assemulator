use pest::{Parser, iterators::Pairs};

/// Pest parser for the assembler.
#[derive(pest_derive::Parser)]
#[grammar = "./src/assembler/grammar.pest"]
struct AsmParser;

pub fn parse(text: &str) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
    AsmParser::parse(Rule::lines, text)
}
