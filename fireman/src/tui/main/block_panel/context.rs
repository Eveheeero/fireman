pub struct Context {
    list: Vec<String>,
    input: String,
}

impl Context {
    pub fn new() -> Self {
        Context {
            list: Vec::new(),
            input: String::new(),
        }
    }
}
