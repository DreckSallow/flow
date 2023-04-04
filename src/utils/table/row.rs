use super::utils;

pub struct Row {
    cells: Vec<RowCell>,
    max_height: u8,
}

impl Row {
    pub fn new(cells: Vec<RowCell>) -> Self {
        let mut max_height = 1;

        for cell in &cells {
            let cell_height = cell.get_sizes().0;

            if cell_height > max_height {
                max_height = cell_height
            }
        }

        Self { cells, max_height }
    }
    pub fn get_text(&mut self, each_col: &Vec<((u8, u8), u8)>) -> String {
        let mut row_text = String::new();

        let cells = {
            let mut cells = vec![];
            for cell in &self.cells {
                let mut lines = cell.get_lines();
                let max_width = cell.get_sizes().1;

                let missing = self.max_height - lines.len() as u8;

                for _i in 0..missing {
                    let s = utils::num_to_str(&max_width, " ");
                    lines.push((s.clone(), s.chars().count() as u8))
                }

                cells.push(lines);
            }
            cells
        };

        for row in 0..self.max_height as usize {
            let mut internal_row = String::new();
            internal_row.push_str("|");

            for col in 0..cells.len() {
                //Walk over cells
                let (padd, max) = each_col[col];
                let cell = &cells[col][row];

                internal_row.push_str(&utils::num_to_str(&padd.0, " "));
                internal_row.push_str(&cell.0);
                internal_row.push_str(&utils::num_to_str(&(max - cell.1), " "));
                internal_row.push_str(&utils::num_to_str(&padd.1, " "));
                internal_row.push_str("|");
            }
            row_text.push_str(&internal_row);
            row_text.push_str("\n");
        }

        row_text
    }
}

impl From<Vec<String>> for Row {
    fn from(cells: Vec<String>) -> Self {
        let cells_to_row = cells.iter().map(|s| RowCell::from(s)).collect();
        Self::new(cells_to_row)
    }
}

#[derive(Clone, Debug)]
pub enum RowCell {
    Multiple(Vec<String>),
    Single(String),
    Styled(String, String),
}

impl RowCell {
    /// Return a tuple follow this: ( height, width )
    pub fn get_sizes(&self) -> (u8, u8) {
        match self {
            RowCell::Multiple(l) => {
                let height = l.len() as u8;
                let width = l.iter().fold(0, |init, l| {
                    let len = l.chars().count();
                    if len > init {
                        len
                    } else {
                        init
                    }
                });
                (height, width as u8)
            }
            RowCell::Single(l) => (1, l.chars().count() as u8),
            RowCell::Styled(_, s) => (1, s.chars().count() as u8),
        }
    }
    pub fn get_lines(&self) -> Vec<(String, u8)> {
        match self {
            RowCell::Multiple(l) => {
                let mut each = vec![];
                for c in l {
                    each.push((c.clone(), c.chars().count() as u8))
                }

                each
            }
            RowCell::Single(l) => {
                vec![(l.to_string(), l.chars().count() as u8)]
            }
            RowCell::Styled(l, s) => vec![(l.to_string(), s.chars().count() as u8)],
        }
    }

    pub fn is_multiple(&self) -> bool {
        match self {
            RowCell::Multiple(_) => true,
            _ => false,
        }
    }
}

impl<T> From<T> for RowCell
where
    T: ToString,
{
    fn from(t: T) -> Self {
        let lines: Vec<String> = t.to_string().lines().map(|l| l.to_owned()).collect();
        if lines.len() > 1 {
            RowCell::Multiple(lines)
        } else {
            RowCell::Single(lines[0].clone())
        }
    }
}
