use std::collections::VecDeque;

pub trait TakeErrorsExt: Iterator {
    type Passed: Iterator;
    type Errors;

    fn take_errors(self) -> Result<Self::Passed, Self::Errors>;
}

impl<R, E, I: Iterator<Item=Result<R, E>>> TakeErrorsExt for I { 
    type Passed = TakeErrorsIter<R>;
    type Errors = Vec<E>;

    fn take_errors(self) -> Result<Self::Passed, Self::Errors> {
        let mut errors = Vec::new();
        let mut success = VecDeque::new();
        for val in self {
            match val {
                Err(error) => errors.push(error),
                Ok(ok) => success.push_back(ok),
            }
        }
        if errors.is_empty() {
            Ok(TakeErrorsIter { internal: success })
        } else {
            Err(errors)
        }
    }
}

pub struct TakeErrorsIter<I> {
    internal: VecDeque<I>,
}

impl<I> Iterator for TakeErrorsIter<I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.pop_front()
    }
}