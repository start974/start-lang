use super::stateResult::StateResult;

pub trait State: Sized {
    type Input;
    type Output;
    type Error;

    /// fn feed input
    fn add_input(&mut self, input: Self::Input);

    /// fn with input
    fn with_input(mut self, input: Self::Input) -> Self {
        self.add_input(input);
        self
    }

    /// get result
    fn run(&mut self) -> StateResult<Self::Output, Self::Error>;
}
