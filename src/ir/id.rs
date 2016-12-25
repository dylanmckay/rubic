use std::marker;

static mut ACCUMULATOR: u64 = 0;

/// A unique identifier.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Id<T> {
    id: u64,
    phantom: marker::PhantomData<T>,
}

impl<T> Id<T>
{
    /// Creates a new unique identifier.
    pub fn new() -> Self {
        unsafe {
            ACCUMULATOR += 1;
            Id { id: ACCUMULATOR, phantom: marker::PhantomData }
        }
    }
}
