use std::collections::HashMap;

use crate::mask;

use super::{Error, Token, expression, parser::Rule, template::Template};

pub fn interpret_escaped_chars(text: &str) -> String {
    let mut out = String::new();
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('\\') | None => out.push('\\'),
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some('\'') => out.push('\''),
                Some('\"') => out.push('\"'),
                Some('0') => out.push('\0'),
                Some(x) => out.push(x),
            }
        } else {
            out.push(c);
        }
    }
    out
}

enum LabelType {
    /// Address in program memory.
    Address(u64),
    /// Address in data memory.
    Data(u64),
    /// Constant declared with `.set`.
    Value(u64),
    Template(Template),
}

impl LabelType {
    /// The address of a label that marks a location in memory.
    const fn location(&self) -> Option<u64> {
        match self {
            Self::Address(x) | Self::Data(x) => Some(*x),
            Self::Value(_) | Self::Template(_) => None,
        }
    }
}

impl std::fmt::Debug for LabelEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.label {
            LabelType::Address(x) => write!(f, "Address({x})"),
            LabelType::Data(x) => write!(f, "Data({x})"),
            LabelType::Value(x) => write!(f, "Value({x})"),
            LabelType::Template(_) => write!(f, "Template(...)"),
        }
    }
}

struct LabelEntry {
    idx: usize,
    label: LabelType,
}

#[derive(Default)]
pub struct SymbolTable {
    symbols: HashMap<String, LabelEntry>,
    label_prefix: Option<String>,
    label_index: usize,
    is_final_pass: bool,
}

fn is_local_label(label: &str) -> bool {
    label.rsplit("::").next().unwrap().starts_with('.')
}

impl SymbolTable {
    pub fn get_main(&self) -> Option<u64> {
        let &LabelEntry {
            label: LabelType::Address(address),
            ..
        } = self.symbols.get("main")?
        else {
            return None;
        };
        Some(address)
    }
    pub const fn new_pass(&mut self, is_final_pass: bool) {
        self.label_index = 0;
        self.is_final_pass = is_final_pass;
    }

    fn resolve_label(&self, label: &str) -> String {
        self.label_prefix
            .as_ref()
            .map_or_else(|| label.to_string(), |prefix| format!("{prefix}::{label}"))
    }

    fn add_entry(&mut self, label: &Token, value: LabelEntry) -> Result<(), Error> {
        let mut label_str = self.resolve_label(label.as_str());

        if is_local_label(&label_str) {
            label_str = format!("{label_str}-{}", self.label_index);
        }

        if let Some(entry) = self.symbols.get(&label_str) {
            let moved = entry.label.location() != value.label.location();
            if entry.idx != self.label_index || moved {
                // A real duplicate is caught in the first pass, so in the final pass
                // this means the layout changed after forward references resolved.
                let msg = if self.is_final_pass {
                    "label moved between passes: something above it depends on a label defined later"
                } else {
                    "label already defined"
                };
                return Err(Error::at(label, msg));
            }
        }

        self.symbols.insert(label_str, value);
        self.label_index += 1;

        Ok(())
    }

    pub fn declare_program_label(
        &mut self,
        label: Option<Token>,
        address: u64,
    ) -> Result<(), Error> {
        let Some(label) = label else { return Ok(()) };

        let entry = LabelEntry {
            idx: self.label_index,
            label: LabelType::Address(address),
        };
        self.add_entry(&label, entry)
    }

    pub fn declare_data_label(&mut self, label: Option<Token>, address: u64) -> Result<(), Error> {
        let Some(label) = label else { return Ok(()) };

        let entry = LabelEntry {
            idx: self.label_index,
            label: LabelType::Data(address),
        };
        self.add_entry(&label, entry)
    }

    pub fn declare_value(&mut self, label: Option<Token>, value: u64) -> Result<(), Error> {
        let Some(label) = label else { return Ok(()) };

        let entry = LabelEntry {
            idx: self.label_index,
            label: LabelType::Value(value),
        };
        self.add_entry(&label, entry)
    }

    pub fn declare_macro(&mut self, label: Option<Token>, template: Template) -> Result<(), Error> {
        let Some(label) = label else { return Ok(()) };

        let entry = LabelEntry {
            idx: self.label_index,
            label: LabelType::Template(template),
        };

        self.add_entry(&label, entry)
    }

    pub fn get_macro(&self, label: &str) -> Option<&Template> {
        let value = self.symbols.get(label)?;

        let LabelType::Template(ref template) = value.label else {
            return None;
        };

        Some(template)
    }

    fn get_local_label(&self, label: &str) -> Option<&LabelEntry> {
        let mut symbols = self.symbols.iter().collect::<Vec<_>>();
        symbols.sort_by_key(|&(_, v)| v.idx);

        let start = symbols
            .binary_search_by_key(&self.label_index.checked_sub(1)?, |x| x.1.idx)
            .ok()?
            + 1;

        let local_labels_above = symbols[..start]
            .iter()
            .rev()
            .take_while(|&(key, _)| is_local_label(key))
            .find(|x| x.0.split('-').next().unwrap() == label)
            .map(|x| x.1);

        let local_labels_below = symbols[start..]
            .iter()
            .take_while(|&(key, _)| is_local_label(key))
            .find(|x| x.0.split('-').next().unwrap() == label)
            .map(|x| x.1);

        local_labels_above.or(local_labels_below)
    }

    fn get_label(&self, label: &Token) -> Result<u64, Error> {
        let label_str = self.resolve_label(label.as_str());

        let entry = if is_local_label(&label_str) {
            self.get_local_label(&label_str)
        } else {
            self.symbols.get(&label_str)
        };

        match entry {
            Some(LabelEntry {
                idx: _,
                label: LabelType::Address(x) | LabelType::Data(x) | LabelType::Value(x),
            }) => Ok(*x),
            _ => Err(Error::at(label, "undefined label")),
        }
    }

    /// Evaluates an expression argument, substituting labels and characters
    /// with their values.
    ///
    /// Returns an error if a label is undefined or the value doesn't fit in `U`.
    pub fn expression<U>(&self, arg: &Token) -> Result<U, Error>
    where
        U: TryFrom<u64> + Copy,
    {
        let value = self.evaluate(arg, None)?;
        let n_bits = 8 * std::mem::size_of::<U>();
        mask(value, n_bits).map_err(|err| Error::at(arg, &err))
    }

    /// Evaluates the offset of a memory operand like `[arr + x]`, where
    /// `register` names its base register.
    pub fn memory_offset(&self, arg: &Token, register: &str) -> Result<u64, Error> {
        let offset = self.evaluate(arg, Some((register, 0)))?;
        // the register must be added to the offset, e.g. `[arr + x]` but not `[x * 2]`
        for probe in [1, 0x1234_5678] {
            if self.evaluate(arg, Some((register, probe)))? != offset.wrapping_add(probe) {
                return Err(Error::at(
                    arg,
                    "a register in a memory operand can only be added to the offset",
                ));
            }
        }
        Ok(offset)
    }

    /// Evaluates an expression, substituting `register` with its given value.
    fn evaluate(&self, arg: &Token, register: Option<(&str, u64)>) -> Result<u64, Error> {
        let text = arg.as_str();
        let offset = arg.as_span().start();
        let mut full_expr = String::new();
        let mut last = 0;
        for symbol in arg.clone().into_inner() {
            let (start, end) = (
                symbol.as_span().start() - offset,
                symbol.as_span().end() - offset,
            );

            let (start, end, value) = match symbol.as_rule() {
                Rule::label => match register {
                    Some((name, value)) if symbol.as_str() == name => (start, end, value),
                    _ => (start, end, self.get_label(&symbol)?),
                },
                Rule::character => {
                    let inner = symbol.clone().into_inner().as_str();
                    let c = interpret_escaped_chars(inner);
                    (start, end, u64::from(c.chars().next().unwrap_or_default()))
                }
                Rule::number => continue,
                Rule::string => {
                    return Err(Error::at(&symbol, "expected a number, not a string"));
                }
                _ => return Err(Error::at(&symbol, "unexpected token")),
            };
            full_expr.push_str(&text[last..start]);
            full_expr.push_str(&value.to_string());
            last = end;
        }
        full_expr.push_str(&text[last..]);
        full_expr.retain(|c| c != ' ');

        expression::parse(&full_expr).map_err(|err| Error::at(arg, &err))
    }

    /// Replace prefix.
    pub fn new_file(&mut self, prefix: Option<Token>) -> Option<String> {
        let ret = self.label_prefix.take();
        self.label_prefix = prefix.map(|tk| tk.as_str().to_string());
        ret
    }

    /// Restore prefix.
    pub fn done_with_file(&mut self, prefix: Option<String>) {
        self.label_prefix = prefix;
    }
}
