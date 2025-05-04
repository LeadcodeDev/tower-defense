use super::ports::notifier::Notifier;

pub struct Mediator<T: Notifier> {
    pub notifier: T,
}

impl<T: Notifier> Mediator<T> {
    pub fn new(notifier: T) -> Self {
        Self { notifier }
    }
}
