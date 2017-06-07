use std::marker::Sized;

pub trait TryFrom<T> where Self: Sized {
    type Err;
    fn try_from(value: T) -> Result<Self, Self::Err>;
}
