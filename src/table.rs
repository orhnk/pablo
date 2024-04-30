/// Type representing a periodic table.
/// Defaults to standard 9x18

type Element = String;
pub struct Table<'a> {
    /// Method defining the fine shape
    repr: &'a [&'a [bool]],

    /// Instances sorted accordingly to fit the representation
    order: &'a [Element],
}

impl<'a> Table<'a> {
    pub fn new(repr: &'a [&'a [bool]], order: &'a [Element]) -> Self {
        Self { repr, order }
    }
}
