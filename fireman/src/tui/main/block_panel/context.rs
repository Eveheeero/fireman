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

    pub fn get_input(&self) -> &String {
        &self.input
    }
    pub fn get_input_mut(&mut self) -> &mut String {
        &mut self.input
    }
}
