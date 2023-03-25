pub mod row;
mod utils;

use row::Row;

pub use row::RowCell;

pub struct Table {
    pub headers: Row,
    pub body: Vec<Row>,
    pub max_columns: u8,
    contains_multiple: bool,
    each_column_max: Vec<u8>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            body: vec![],
            contains_multiple: false,
            headers: Row::new(vec![]),
            each_column_max: vec![],
            max_columns: 0,
        }
    }

    pub fn add_headers(&mut self, headers: Vec<&str>) {
        self.max_columns = headers.len() as u8;

        self.each_column_max = headers
            .iter()
            .map(|header| header.len() as u8)
            .collect::<Vec<u8>>();

        self.headers = Row::new(
            headers
                .iter()
                .map(|r| RowCell::Single((*r).to_owned()))
                .collect(),
        );
    }

    pub fn insert_row(&mut self, row: Vec<RowCell>) {
        if (row.len() as u8) < self.max_columns {
            eprintln!("The row length is less than the headers : {:?}", row);
            panic!("Row length is short")
        }
        // Get the max of columns in the table, previous insert with the headers
        let slice_row = row[0..self.max_columns as usize].to_vec();

        // Sync the max length for each column
        for (i, row_cell) in slice_row.iter().enumerate() {
            if row_cell.is_multiple() {
                self.contains_multiple = true;
            }
            let chars_length = row_cell.get_sizes().1;
            if (chars_length as u8) > self.each_column_max[i] {
                self.each_column_max[i] = chars_length as u8;
            }
        }

        self.body.push(Row::new(slice_row));
    }

    pub fn get_table(&mut self, min_padd: u8) -> String {
        let mut table_text = String::new();
        let row_max = self.calc_max(min_padd);
        let save_line_break = self.get_break_line(&row_max);

        let headers_text = self.headers.get_text(&row_max);
        table_text.push_str(&save_line_break);
        table_text.push_str("\n");

        table_text.push_str(&headers_text);
        table_text.push_str(&save_line_break);
        table_text.push_str("\n");

        for body_row in self.body.iter_mut() {
            let body_row_text = body_row.get_text(&row_max);
            table_text.push_str(&body_row_text);

            if self.contains_multiple {
                table_text.push_str(&save_line_break);
                table_text.push_str("\n");
            }
        }
        if !self.contains_multiple {
            table_text.push_str(&save_line_break);
        }

        return table_text;
    }

    pub fn get_break_line(&self, each_col: &Vec<((u8, u8), u8)>) -> String {
        let mut line_break = String::new();
        let mut i = 0;

        for (padds, col) in each_col {
            if i == 0 {
                line_break.push_str("+");
            } else {
                line_break.push_str(" +");
            }
            let col_len = col + padds.0 + padds.1;
            line_break.push_str(&utils::num_to_str(&((col_len) / 2), " -"));
            i += 1;
        }
        line_break.push_str(" +");

        line_break
    }

    fn calc_max(&self, min_padd: u8) -> Vec<((u8, u8), u8)> {
        let mut max_cols_len = Vec::new();

        for max_col in &self.each_column_max {
            let mut max = max_col + (min_padd * 2); // [left] + [col] + [right]
            if max % 2 != 0 {
                let floor_n = ((max / 2) as f64).floor() as u8;
                max = (floor_n + 1) * 2;
            }
            let missing = max - max_col;
            let paddings = match missing % 2 {
                0 => (missing / 2, missing / 2), //If is multiple of two: [left] {} [right]
                _ => {
                    let down_n = ((missing / 2) as f64).floor();
                    (down_n as u8, missing - down_n as u8)
                }
            };
            max_cols_len.push((paddings, (max + 1) - (paddings.0 + paddings.1)));
        }
        max_cols_len
    }
}
