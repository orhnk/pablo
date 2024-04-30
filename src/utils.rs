// Algo:
// a grid of squares to draw the elements on (standard: 9x18)
// a recipe representing fine shape of the periodic table (not a 9x18 square)
// an order to fullfill the table

pub mod ptable_elements {
    pub const BLOCK: &str = "##";
    pub const SPACE: &str = "  ";
    pub const OFFSET: usize = {
        if BLOCK.len() == SPACE.len() {
            BLOCK.len()
        } else { // panicing here is unstable ~(2024-04-30)
            0 // illegal
        }
    };
}

pub mod ptable_repr {
    pub const STANDARD: &[&str] = &[
        "##                                ##",
        "####                    ############",
        "####                    ############",
        "####################################",
        "####################################",
        "####  ##############################",
        "####  ##############################",
        "                                    ",
        "    ##############################  ",
        "    ##############################  ",
    ];
}

pub mod prep {
    /// Preproccesses a periodic table into boolean arrays
    pub fn table_to_bool<'a>(repr: &'a [&str]) -> Vec<Vec<bool>> {
        let mut bool_ptable = vec![vec![false; repr[0].len() / 2]; repr.len()]; // initialize our table with falses
        let block_size = super::ptable_elements::OFFSET;
        for (i, period) in repr.iter().enumerate() {
            let mut j = 0;
            while j*block_size < period.len() {
                if &period[j*block_size..j*block_size + 2] == "##" {
                    bool_ptable[i][j] = true;
                }
                j += 1;
            }
        }
        bool_ptable
    }
} /* prep */
