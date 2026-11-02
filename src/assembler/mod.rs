use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

pub use error::Error;
use parser::{Rule, parse};
use pest::Position;
use pest::iterators::{Pair, Pairs};
use symbol_table::{SymbolTable, interpret_escaped_chars};
use template::Template;

use crate::{Argument, Endian, InstructionSet};

mod error;
mod expression;
mod parser;
mod symbol_table;
mod template;
#[cfg(test)]
mod tests;

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

pub struct Assembler<T> {
    program: Vec<u8>,
    data: Vec<u8>,
    symbol_table: SymbolTable,
    is_final_pass: bool,
    /// Offsets of the `.if` blocks open in the file or macro being assembled.
    open_ifs: Vec<usize>,
    included_files: HashSet<PathBuf>,
    phantom: std::marker::PhantomData<T>,
}

type Token<'a> = Pair<'a, Rule>;

pub struct AssemblyResult {
    pub start: u64,
    pub program: Vec<u8>,
    pub data: Vec<u8>,
}

// not derived, which would require `T: Default`
impl<T> Default for Assembler<T> {
    fn default() -> Self {
        Self {
            program: Vec::new(),
            data: Vec::new(),
            symbol_table: SymbolTable::default(),
            is_final_pass: false,
            open_ifs: Vec::new(),
            included_files: HashSet::new(),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<T: InstructionSet> Assembler<T> {
    pub fn assemble(path: &Path) -> Result<AssemblyResult, Error> {
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

    fn assemble_instruction(&mut self, path: &Path, pair: &Token) -> Result<(), Error> {
        let mut tokens = pair.clone().into_inner();

        let opcode = tokens.next().unwrap();

        if let Some(template) = self.symbol_table.get_macro(opcode.as_str()) {
            let args = tokens.map(|x| x.as_str()).collect::<Vec<_>>();
            if args.len() != template.arity() {
                let msg = format!(
                    "macro expects {} arguments, got {}",
                    template.arity(),
                    args.len()
                );
                return Err(Error::at(pair, &msg));
            }
            let body = template.expand(&args);
            let definition = template.path().to_owned();

            self.parse_lines(body, &definition).map_err(|error| {
                let call = Error::at(pair, "in expansion of this macro").in_file(path);
                Error::Message(format!("{error}\n{call}"))
            })?;
        } else if let Ok(opcode) = T::Opcode::try_from(opcode.as_str()) {
            let mut arguments = vec![];
            for arg in tokens {
                let argument = if arg.as_rule() == Rule::memory {
                    self.memory_operand(&arg)?
                } else if let Ok(reg) = T::Register::try_from(arg.as_str().trim()) {
                    Argument::Reg(reg)
                } else {
                    Argument::Imm(self.expression::<u64>(&arg)?)
                };
                arguments.push(argument);
            }

            let bytes = match T::encode(self.program.len() as u64, opcode, &arguments) {
                Ok(bytes) => bytes,
                // operands may hold placeholders for labels that aren't defined yet,
                // so report in the final pass, where an undefined label shows as such
                Err(_) if !self.is_final_pass => Vec::new(),
                Err(err) => return Err(Error::at(pair, &err)),
            };

            self.program.extend(bytes);
        } else {
            Err(Error::at(&opcode, "unknown opcode"))?;
        }

        Ok(())
    }

    fn assemble_directive(
        &mut self,
        pair: &Token,
        path: &Path,
        label: Option<Token>,
        lines: &mut Pairs<Rule>,
    ) -> Result<(), Error> {
        let mut tokens = pair.clone().into_inner();

        let command = tokens.next().unwrap();
        match command.as_str() {
            "abort" => {
                Err(Error::at(pair, "abort called"))?;
            }
            "macro" => {
                let args = tokens.map(|x| x.as_str().to_owned()).collect::<Vec<_>>();

                // keep each line of the body at its line and column in the source,
                // so errors in an expansion point at the definition
                let mut body = String::new();
                let mut line = 1;
                let mut closed = false;
                for pair in lines.by_ref() {
                    if directive_name(&pair) == Some("endm") {
                        closed = true;
                        break;
                    }
                    let (pair_line, column) = pair.line_col();
                    if pair_line > line {
                        body.push_str(&"\n".repeat(pair_line - line));
                        body.push_str(&" ".repeat(column - 1));
                        line = pair_line;
                    } else {
                        body.push(' ');
                    }
                    body.push_str(pair.as_str());
                }
                if !closed {
                    return Err(Error::at(pair, "missing .endm"));
                }

                let template = Template::new(args, body, path.to_owned());
                self.declare_macro(label, template)?;
            }
            "set" => {
                let arg = tokens
                    .next()
                    .ok_or_else(|| Error::at(&command, "Expected expression"))?;
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
                    .ok_or_else(|| Error::at(&command, "Expected filename"))?;
                let filename = path
                    .parent()
                    .unwrap_or_else(|| Path::new("."))
                    .join(filename.as_str());

                if self.included_files.contains(&filename) && label.is_none() {
                    return Ok(());
                }
                self.included_files.insert(filename.clone());

                let text = std::fs::read_to_string(&filename).map_err(|err| {
                    let message = format!("can't read {}: {err}", filename.display());
                    Error::at(pair, &message)
                })?;
                let old_prefix = self.symbol_table.new_file(label);
                self.parse_lines(text, &filename)?;
                self.symbol_table.done_with_file(old_prefix);
            }
            "zero" => {
                let arg = tokens
                    .next()
                    .ok_or_else(|| Error::at(&command, "Expected count"))?;
                let count = self.expression::<usize>(&arg)?;
                if count > 256 {
                    Err(Error::at(&arg, "bad count"))?;
                }
                self.declare_data_label(label)?;
                self.data.resize(self.data.len() + count, 0);
            }
            "strz" => {
                // null-terminated string
                self.declare_data_label(label)?;
                for pair in tokens {
                    if pair.as_rule() != Rule::string {
                        return Err(Error::at(&pair, "expected a string"));
                    }
                    let parsed = interpret_escaped_chars(pair.as_str());
                    self.data.extend(parsed.as_bytes());
                    self.data.push(0);
                }
            }
            _ => Err(Error::at(&command, "unknown directive"))?,
        }
        Ok(())
    }

    /// Assembles `.i8`, `.u8`, `.i16` and `.u16`.
    fn data_directive(&mut self, name: &str, tokens: Pairs<Rule>) -> Result<(), Error> {
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
                    .map_err(|err| Error::at(&arg, &format!("{err} for .{name}")))?;
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
    ) -> Result<(), Error> {
        match name {
            "if" => {
                let expr = tokens
                    .next()
                    .ok_or_else(|| Error::at(pair, "Expected expression"))?;

                let value = self.expression::<u64>(&expr)?;
                if value != 0 {
                    self.open_ifs.push(pair.as_span().start());
                } else {
                    match skip_block(lines, true) {
                        Some(BlockEnd::Else) => self.open_ifs.push(pair.as_span().start()),
                        Some(BlockEnd::Endif) => {}
                        None => return Err(Error::at(pair, "missing .endif")),
                    }
                }
            }
            "else" => {
                // reached the end of a taken `.if` branch
                if self.open_ifs.pop().is_none() {
                    return Err(Error::at(pair, ".else without .if"));
                }
                if skip_block(lines, false).is_none() {
                    return Err(Error::at(pair, "missing .endif"));
                }
            }
            "endif" => {
                if self.open_ifs.pop().is_none() {
                    return Err(Error::at(pair, ".endif without .if"));
                }
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    fn declare_data_label(&mut self, label: Option<Token>) -> Result<(), Error> {
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

    fn declare_program_label(&mut self, label: Option<Token>) -> Result<(), Error> {
        self.symbol_table
            .declare_program_label(label, self.program.len() as u64)
    }

    fn declare_macro(&mut self, label: Option<Token>, template: Template) -> Result<(), Error> {
        self.symbol_table.declare_macro(label, template)
    }

    fn expression<U>(&self, arg: &Token) -> Result<U, Error>
    where
        U: TryFrom<u64> + Copy + Default,
    {
        match self.symbol_table.expression::<U>(arg) {
            Ok(value) => Ok(value),
            Err(_) if !self.is_final_pass => Ok(U::default()),
            Err(err) => Err(err),
        }
    }

    /// Parses a memory operand like `[arr + x]` into its base register and offset.
    fn memory_operand(&self, arg: &Token) -> Result<Argument<T::Register>, Error> {
        let expr = arg.clone().into_inner().next().unwrap();
        let mut registers = expr
            .clone()
            .into_inner()
            .filter(|x| x.as_rule() == Rule::label)
            .filter_map(|x| T::Register::try_from(x.as_str()).ok().map(|reg| (reg, x)));

        let base = registers.next();
        if let Some((_, extra)) = registers.next() {
            return Err(Error::at(
                &extra,
                "a memory operand can only use one register",
            ));
        }

        let offset = match &base {
            Some((_, name)) => self.symbol_table.memory_offset(&expr, name.as_str()),
            None => self.symbol_table.expression::<u64>(&expr),
        };
        let offset = match offset {
            Ok(offset) => offset,
            Err(_) if !self.is_final_pass => 0,
            Err(err) => return Err(err),
        };
        Ok(Argument::Mem {
            base: base.map(|(reg, _)| reg),
            offset,
        })
    }

    /// Parses a body of text from `path` into instructions and adds it to the program.
    fn parse_lines(&mut self, text: String, path: &Path) -> Result<(), Error> {
        self.assemble_text(text, path)
            .map_err(|err| err.in_file(path))
    }

    fn assemble_text(&mut self, mut text: String, path: &Path) -> Result<(), Error> {
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
        if let Some(&offset) = self.open_ifs.last() {
            let pos = Position::new(&text, offset).unwrap();
            return Err(Error::at_pos(pos, "missing .endif for this .if"));
        }
        self.open_ifs = outer_ifs;
        Ok(())
    }

    fn do_one_pass(&mut self, path: &Path) -> Result<(), Error> {
        self.symbol_table.new_pass(self.is_final_pass);
        self.included_files.clear();
        self.program.clear();
        self.data.clear();
        let text = std::fs::read_to_string(path)
            .map_err(|err| Error::Message(format!("can't read {}: {err}", path.display())))?;
        self.parse_lines(text, path)
    }
}
