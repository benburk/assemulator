#[derive(Debug, Clone)]
pub struct Template {
    /// List of arguments.
    args: Vec<String>,
    /// List of instructions.
    body: String,
}

impl Template {
    pub fn new(args: Vec<String>, body: String) -> Self {
        Self { args, body }
    }

    pub fn expand(&self, values: &[&str]) -> String {
        let mut body = self.body.clone();
        for (arg, value) in self.args.iter().zip(values) {
            body = body.replace(&format!("${arg}"), value);
        }
        body
    }
}
