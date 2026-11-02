use std::fmt::Write;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use parser::{parse, Rule};
use pest::iterators::{Pair, Pairs};
use symbol_table::{interpret_escaped_chars, SymbolTable};
use template::Template;

use crate::{Argument, Endian, Processor};

mod expression;
mod parser;
mod symbol_table;
mod template;
#[cfg(test)]
mod tests;

/// Pretty prints error.
fn format_error(pair: &Pair<Rule>, msg: &str) -> String {
    pest::error::Error::<Rule>::new_from_span(
        pest::error::ErrorVariant::CustomError {
            message: msg.to_owned(),
        },
        pair.as_span(),
    )
    .to_string()
}

/// Returns the name of a directive, e.g. `if` for `.if x`.
fn directive_name<'a>(pair: &Token<'a>) -> Option<&'a str> {
    (pair.as_rule() == Rule::directive).then(|| pair.clone().into_inner().next().unwrap().as_str())
}

/// The directive that ended a skipped conditional block.
#[derive(PartialEq, Eq)]
enum BlockEnd {
    Else,
    Endif,
}

/// Skips lines up to the `.else` or `.endif` that closes the current `.if`,
/// stepping over nested blocks. Returns `None` at end of input.
fn skip_block(lines: &mut Pairs<Rule>, stop_at_else: bool) -> Option<BlockEnd> {
    let mut depth = 0;
    for line in lines {
        match directive_name(&line) {
            Some("if") => depth += 1,
            Some("else") if depth == 0 && stop_at_else => return Some(BlockEnd::Else),
            Some("endif") if depth == 0 => return Some(BlockEnd::Endif),
            Some("endif") => depth -= 1,
            _ => {}
        }
    }
    None
}

/// Checks that a value fits in a signed or unsigned field of `bytes` bytes.
/// Values with the top bit set are treated as negative.
fn check_range(value: u64, bytes: usize, signed: bool) -> Result<(), String> {
    let value = value.cast_signed();
    let bits = 8 * bytes;
    let (min, max) = if signed {
        (-(1_i64 << (bits - 1)), (1_i64 << (bits - 1)) - 1)
    } else {
        (0, (1_i64 << bits) - 1)
    };
    if (min..=max).contains(&value) {
        Ok(())
    } else {
        Err(format!("{value} is outside {min}..={max}"))
    }
}

#[derive(Default)]
pub struct Assembler<T> {
    program: Vec<u8>,
    data: Vec<u8>,
    symbol_table: SymbolTable,
    is_final_pass: bool,
    /// Number of `.if` blocks open in the file or macro being assembled.
    open_ifs: usize,
    included_files: HashSet<PathBuf>,
    phantom: std::marker::PhantomData<T>,
}

type Token<'a> = Pair<'a, Rule>;

pub struct AssemblyResult {
    pub start: u64,
    pub program: Vec<u8>,
    pub data: Vec<u8>,
}

impl<T: Processor> Assembler<T> {
    pub fn assemble(path: &Path) -> Result<AssemblyResult, String> {
        let mut assembler = Self::default();

        assembler.do_one_pass(path)?;
        assembler.is_final_pass = true;
        assembler.do_one_pass(path)?;

        let result = AssemblyResult {
            start: assembler.symbol_table.get_main().unwrap_or_default(),
            program: assembler.program,
            data: assembler.data,
        };
        Ok(result)
    }

    fn assemble_instruction(&mut self, path: &Path, pair: &Token) -> Result<(), String> {
        let mut tokens = pair.clone().into_inner();

        let opcode = tokens.next().unwrap();

        if let Some(template) = self.symbol_table.get_macro(opcode.as_str()) {
            let args = tokens.map(|x| x.as_str()).collect::<Vec<_>>();
            let body = template.expand(&args);

            self.parse_lines(body, path).map_err(|error| {
                format!(
                    "{}\n\n{}",
                    error,
                    format_error(pair, "error in expansion of macro")
                )
            })?;
        } else if let Ok(opcode) = T::Opcode::try_from(opcode.as_str()) {
            let mut arguments = vec![];
            for arg in tokens {
                if let Ok(reg) = T::Register::try_from(arg.as_str().trim()) {
                    arguments.push(Argument::Reg(reg));
                } else {
                    let imm = self.expression::<u64>(&arg)?;
                    arguments.push(Argument::Imm(imm));
                }
            }

            let bytes = T::parse_assembly_line(self.program.len() as u64, opcode, &arguments);
            let bytes = bytes.map_err(|err| format_error(pair, &err))?;

            self.program.extend(bytes);
        } else {
            Err(format_error(&opcode, "unknown opcode"))?;
        }

        Ok(())
    }

    fn assemble_directive(
        &mut self,
        pair: &Token,
        path: &Path,
        label: Option<Token>,
        lines: &mut Pairs<Rule>,
    ) -> Result<(), String> {
        let mut tokens = pair.clone().into_inner();

        let command = tokens.next().unwrap();
        match command.as_str() {
            "abort" => {
                Err(format_error(pair, "abort called"))?;
            }
            "macro" => {
                let args = tokens.map(|x| x.as_str().to_owned()).collect::<Vec<_>>();

                // start the macro on the line it occurs at in the source.
                let mut body = "\n".repeat(command.line_col().0);

                lines
                    .map(|x| x.as_str())
                    .take_while(|x| x.trim() != ".endm")
                    .for_each(|x| writeln!(&mut body, "{x}").unwrap());

                let template = Template::new(args, body);
                self.declare_macro(label, template)?;
            }
            "set" => {
                let arg = tokens
                    .next()
                    .ok_or_else(|| format_error(&command, "Expected expression"))?;
                let value = self.expression::<u64>(&arg)?;
                self.symbol_table.declare_value(label, value)?;
            }
            name @ ("i8" | "u8" | "i16" | "u16") => {
                self.declare_data_label(label)?;
                self.data_directive(name, tokens)?;
            }
            name @ ("if" | "else" | "endif") => {
                self.conditional_directive(name, pair, tokens, lines)?;
            }
            "include" => {
                let filename = tokens
                    .next()
                    .ok_or_else(|| format_error(&command, "Expected filename"))?;
                let filename = path
                    .parent()
                    .unwrap_or_else(|| Path::new("."))
                    .join(filename.as_str());

                if self.included_files.contains(&filename) && label.is_none() {
                    return Ok(());
                }
                self.included_files.insert(filename.clone());

                let old_prefix = self.symbol_table.new_file(label);
                self.include(&filename)?;
                self.symbol_table.done_with_file(old_prefix);
            }
            "zero" => {
                let arg = tokens
                    .next()
                    .ok_or_else(|| format_error(&command, "Expected count"))?;
                let count = self.expression::<usize>(&arg)?;
                if count > 256 {
                    Err(format_error(&arg, "bad count"))?;
                }
                self.declare_data_label(label)?;
                self.data.resize(self.data.len() + count, 0);
            }
            "strz" => {
                // null-terminated string
                self.declare_data_label(label)?;
                for pair in tokens {
                    if pair.as_rule() != Rule::string {
                        return Err(format_error(&pair, "expected a string"));
                    }
                    let parsed = interpret_escaped_chars(pair.as_str());
                    self.data.extend(parsed.as_bytes());
                    self.data.push(0);
                }
            }
            _ => Err(format_error(&command, "unknown directive"))?,
        }
        Ok(())
    }

    /// Assembles `.i8`, `.u8`, `.i16` and `.u16`.
    fn data_directive(&mut self, name: &str, tokens: Pairs<Rule>) -> Result<(), String> {
        let bytes = if name.ends_with('8') { 1 } else { 2 };
        let signed = name.starts_with('i');
        for arg in tokens {
            if arg.as_rule() == Rule::string {
                for c in interpret_escaped_chars(arg.as_str()).bytes() {
                    self.push_data(u64::from(c), bytes);
                }
            } else {
                let value = self.expression::<u64>(&arg)?;
                check_range(value, bytes, signed)
                    .map_err(|err| format_error(&arg, &format!("{err} for .{name}")))?;
                self.push_data(value, bytes);
            }
        }
        Ok(())
    }

    /// Assembles `.if`, `.else` and `.endif`.
    fn conditional_directive(
        &mut self,
        name: &str,
        pair: &Token,
        mut tokens: Pairs<Rule>,
        lines: &mut Pairs<Rule>,
    ) -> Result<(), String> {
        match name {
            "if" => {
                let expr = tokens
                    .next()
                    .ok_or_else(|| format_error(pair, "Expected expression"))?;

                let value = self.expression::<u64>(&expr)?;
                if value != 0 {
                    self.open_ifs += 1;
                } else {
                    match skip_block(lines, true) {
                        Some(BlockEnd::Else) => self.open_ifs += 1,
                        Some(BlockEnd::Endif) => {}
                        None => return Err(format_error(pair, "missing .endif")),
                    }
                }
            }
            "else" => {
                // reached the end of a taken `.if` branch
                if self.open_ifs == 0 {
                    return Err(format_error(pair, ".else without .if"));
                }
                if skip_block(lines, false).is_none() {
                    return Err(format_error(pair, "missing .endif"));
                }
                self.open_ifs -= 1;
            }
            "endif" => {
                if self.open_ifs == 0 {
                    return Err(format_error(pair, ".endif without .if"));
                }
                self.open_ifs -= 1;
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    fn declare_data_label(&mut self, label: Option<Token>) -> Result<(), String> {
        self.symbol_table
            .declare_data_label(label, self.data.len() as u64)
    }

    /// Appends the low `bytes` bytes of `value` in the processor's byte order.
    fn push_data(&mut self, value: u64, bytes: usize) {
        match T::ENDIAN {
            Endian::Big => self.data.extend(&value.to_be_bytes()[8 - bytes..]),
            Endian::Little => self.data.extend(&value.to_le_bytes()[..bytes]),
        }
    }

    fn declare_program_label(&mut self, label: Option<Token>) -> Result<(), String> {
        self.symbol_table
            .declare_program_label(label, self.program.len() as u64)
    }

    fn declare_macro(&mut self, label: Option<Token>, template: Template) -> Result<(), String> {
        self.symbol_table.declare_macro(label, template)
    }

    fn expression<U>(&self, arg: &Token) -> Result<U, String>
    where
        U: TryFrom<u64> + Copy + Default,
    {
        match self.symbol_table.expression::<U>(arg) {
            Ok(value) => Ok(value),
            Err(_) if !self.is_final_pass => Ok(U::default()),
            Err(err) => Err(err),
        }
    }

    /// Parses a body of text into instructions and adds it to the program.
    fn parse_lines(&mut self, mut text: String, path: &Path) -> Result<(), String> {
        // conditional blocks can't span files or macros
        let outer_ifs = std::mem::take(&mut self.open_ifs);
        text.push('\n');
        let mut lines = parse(&text)?;
        let mut label = None;
        while let Some(pair) = lines.next() {
            match pair.as_rule() {
                Rule::label_declaration => {
                    // declare loose label as program label
                    self.declare_program_label(label.take())?;
                    label = Some(pair.into_inner().next().unwrap());
                }
                Rule::directive => {
                    self.assemble_directive(&pair, path, label.take(), &mut lines)?;
                }
                Rule::instruction => {
                    self.declare_program_label(label.take())?;
                    self.assemble_instruction(path, &pair)?;
                }
                Rule::EOI => {}
                _ => unreachable!(),
            }
        }
        // declare dangling label for instructions
        self.declare_program_label(label.take())?;
        if self.open_ifs != 0 {
            return Err(format!("missing .endif in {}", path.display()));
        }
        self.open_ifs = outer_ifs;
        Ok(())
    }

    fn include(&mut self, path: &Path) -> Result<(), String> {
        let text = std::fs::read_to_string(path)
            .map_err(|_| format!("Failed to read file {}", path.display()))?;

        self.parse_lines(text, path)?;
        Ok(())
    }

    fn do_one_pass(&mut self, path: &Path) -> Result<(), String> {
        self.symbol_table.new_pass(self.is_final_pass);
        self.included_files.clear();
        self.program.clear();
        self.data.clear();
        self.include(path)
    }
}
