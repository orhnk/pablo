/// Type representing a periodic table.
/// Defaults to standard 9x18

type Element = String;
pub struct Table<const width: usize = 9, const height: usize = 18 > {
    /// Method defining the fine shape
    repr: [[bool; width]; height],

    /// Instances sorted accordingly to fit the representation
    content: Vec<Element>,
}