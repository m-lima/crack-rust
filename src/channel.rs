pub trait Channel: Copy + Clone + Send + 'static {
    fn progress(&self, progress: u8);
    fn result(&self, input: &str, output: &str);
    fn should_terminate(&self) -> bool;
}
