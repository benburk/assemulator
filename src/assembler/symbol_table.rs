use std::collections::HashMap;

use crate::mask;

use super::{expression, format_error, parser::Rule, template::Template, Token};

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
    Address(u64),
    Value(u64),
    Template(Template),
}

impl std::fmt::Debug for LabelEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.label {
            LabelType::Address(x) => write!(f, "Address({x})"),
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
    pub fn new_pass(&mut self) {
        self.label_index = 0;
    }

    fn resolve_label(&self, label: &str) -> String {
        match &self.label_prefix {
            Some(prefix) => format!("{prefix}::{label}"),
            None => label.to_string(),
        }
    }

    fn add_entry(&mut self, label: &Token, value: LabelEntry) -> Result<(), String> {
        let mut label_str = self.resolve_label(label.as_str());

        if is_local_label(&label_str) {
            label_str.push_str(&format!("-{}", self.label_index));
        } else if let Some(entry) = self.symbols.get(&label_str) {
            if entry.idx != self.label_index {
                return Err(format_error(label, "label already defined"));
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
    ) -> Result<(), String> {
        let Some(label) = label else { return Ok(()) };

        let entry = LabelEntry {
            idx: self.label_index,
            label: LabelType::Address(address),
        };
        self.add_entry(&label, entry)
    }

    pub fn declare_value(&mut self, label: Option<Token>, value: u64) -> Result<(), String> {
        let Some(label) = label else { return Ok(()) };

        let entry = LabelEntry {
            idx: self.label_index,
            label: LabelType::Value(value),
        };
        self.add_entry(&label, entry)
    }

    pub fn declare_macro(
        &mut self,
        label: Option<Token>,
        template: Template,
    ) -> Result<(), String> {
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
            .binary_search_by_key(&(self.label_index - 1), |x| x.1.idx)
            .ok()?
            + 1;

        let things_above = symbols[..start]
            .iter()
            .rev()
            .take_while(|&(key, _)| is_local_label(key))
            .find(|x| x.0.split('-').next().unwrap() == label)
            .map(|x| x.1);

        let things_below = symbols[start..]
            .iter()
            .take_while(|&(key, _)| is_local_label(key))
            .find(|x| x.0.split('-').next().unwrap() == label)
            .map(|x| x.1);

        let candidate = things_above.or(things_below);

        candidate
    }

    fn get_label(&self, label: &Token) -> Result<u64, String> {
        let label_str = self.resolve_label(label.as_str());

        let entry = if is_local_label(&label_str) {
            self.get_local_label(&label_str)
        } else {
            self.symbols.get(&label_str)
        };

        match entry {
            Some(LabelEntry {
                idx: _,
                label: LabelType::Address(x) | LabelType::Value(x),
            }) => Ok(*x),
            _ => Err(format_error(label, "undefined label")),
        }
    }

    /// Returns 0 if a label is not defined.
    pub fn expression<U>(&mut self, arg: &Token) -> Result<U, String>
    where
        U: TryFrom<u64> + Copy,
    {
        let mut full_expr = arg.as_str().to_string();
        for symbol in arg.clone().into_inner() {
            let tk = symbol.as_str();

            match symbol.as_rule() {
                Rule::label => {
                    let value = self.get_label(&symbol)?;
                    full_expr = full_expr.replace(tk, &value.to_string());
                }
                Rule::char => {
                    let c = interpret_escaped_chars(tk);
                    let value = u64::from(c.chars().next().unwrap());
                    let value_as_str = value.to_string();
                    full_expr = full_expr.replace(&format!("'{tk}'"), &value_as_str);
                }
                Rule::number => {}
                x => {
                    panic!("unexpeted token: {x:?}");
                }
            }
        }

        full_expr = full_expr.replace(' ', "");

        let value = expression::parse(&full_expr).unwrap();
        let n_bits = 8 * std::mem::size_of::<U>();
        mask(value, n_bits).map_err(|err| format_error(arg, &err))
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
