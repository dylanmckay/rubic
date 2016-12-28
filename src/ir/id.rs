static mut ACCUMULATOR: u64 = 0;

/// A unique identifier.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Id {
    id: u64,
}

impl Id
{
    /// Creates a new unique identifier.
    pub fn new() -> Self {
        unsafe {
            ACCUMULATOR += 1;
            Id { id: ACCUMULATOR }
        }
    }
}
