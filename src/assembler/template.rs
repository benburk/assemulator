#[derive(Debug, Clone)]
pub struct Template {
    /// List of arguments.
    args: Vec<String>,
    /// List of instructions.
    body: String,
}

impl Template {
    pub const fn new(args: Vec<String>, body: String) -> Self {
        Self { args, body }
    }

    pub const fn arity(&self) -> usize {
        self.args.len()
    }

    /// Substitutes each `$name` in the body with the value of that argument.
    /// Names are matched whole, so `$x` doesn't match the start of `$xy`.
    pub fn expand(&self, values: &[&str]) -> String {
        let mut out = String::new();
        let mut rest = self.body.as_str();
        while let Some(i) = rest.find('$') {
            out.push_str(&rest[..i]);
            let after = &rest[i + 1..];
            let len = after
                .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                .unwrap_or(after.len());
            let name = &after[..len];
            if let Some(index) = self.args.iter().position(|arg| arg == name) {
                out.push_str(values[index]);
            } else {
                out.push('$');
                out.push_str(name);
            }
            rest = &after[len..];
        }
        out.push_str(rest);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::Template;

    #[test]
    fn expands_whole_argument_names() {
        let template = Template::new(
            vec!["x".to_owned(), "xy".to_owned()],
            "add $xy, $x+1 ; $other".to_owned(),
        );
        assert_eq!(template.expand(&["r1", "r2"]), "add r2, r1+1 ; $other");
    }
}
