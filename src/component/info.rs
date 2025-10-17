pub struct ComponentInfo {
    id: usize,
    input: Vec<usize>,
    output: Vec<usize>,
}

impl ComponentInfo {
    fn new(id: usize, input: &[usize], output: &[usize]) -> Self {
        Self { id, input: input.to_vec(), output: output.to_vec() }
    }
    fn get_id(&self) -> usize { self.id }
    fn set_id(&mut self, id: usize) { self.id = id }
    fn get_input(&self) -> &[usize] { &self.input }
    fn set_input(&mut self, input: &[usize]) { self.input = input.to_vec() }
    fn get_output(&self) -> &[usize] { &self.output }
    fn set_output(&mut self, output: &[usize]) { self.output = output.to_vec() }
}

impl Default for ComponentInfo {
    fn default() -> Self {
        Self { id: 0, input: Vec::new(), output: Vec::new() }
    }
}