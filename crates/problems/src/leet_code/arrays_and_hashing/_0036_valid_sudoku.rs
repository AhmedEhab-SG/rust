use std::collections::HashSet;

#[allow(unused_imports)]
use utils::Runable;

use crate::Solution;

// problem: https://leetcode.com/problems/valid-sudoku/
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // validate rows
        for (row_idx, row) in board.iter().enumerate() {
            let mut set: HashSet<char> = HashSet::with_capacity(row.len());

            for (col_idx, _) in row.iter().enumerate() {
                let item = board[row_idx][col_idx];

                if item != '.' && !set.insert(item) {
                    return false;
                }

                set.insert(item);
            }
        }

        // validate cols
        for (row_idx, row) in board.iter().enumerate() {
            let mut set: HashSet<char> = HashSet::with_capacity(row.len());

            for (col_idx, _) in row.iter().enumerate() {
                let item = board[col_idx][row_idx];

                if item != '.' && !set.insert(item) {
                    return false;
                }

                set.insert(item);
            }
        }

        // valiate boxs since its 9x9
        for box_row in (0..9).step_by(3) {
            for box_col in (0..9).step_by(3) {
                let mut set = HashSet::new();

                for row in box_row..box_row + 3 {
                    for col in box_col..box_col + 3 {
                        let item = board[row][col];

                        if item != '.' && !set.insert(item) {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }
}

impl Runable for Solution {
    fn run() {
        assert_eq!(
            Self::is_valid_sudoku(
                [
                    ["5", "3", ".", ".", "7", ".", ".", ".", "."],
                    ["6", ".", ".", "1", "9", "5", ".", ".", "."],
                    [".", "9", "8", ".", ".", ".", ".", "6", "."],
                    ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
                    ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
                    ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
                    [".", "6", ".", ".", ".", ".", "2", "8", "."],
                    [".", ".", ".", "4", "1", "9", ".", ".", "5"],
                    [".", ".", ".", ".", "8", ".", ".", "7", "9"]
                ]
                .into_iter()
                .map(|row| row.into_iter().map(|s| s.chars().next().unwrap()).collect())
                .collect()
            ),
            true
        );

        assert_eq!(
            Self::is_valid_sudoku(
                [
                    ["8", "3", ".", ".", "7", ".", ".", ".", "."],
                    ["6", ".", ".", "1", "9", "5", ".", ".", "."],
                    [".", "9", "8", ".", ".", ".", ".", "6", "."],
                    ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
                    ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
                    ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
                    [".", "6", ".", ".", ".", ".", "2", "8", "."],
                    [".", ".", ".", "4", "1", "9", ".", ".", "5"],
                    [".", ".", ".", ".", "8", ".", ".", "7", "9"]
                ]
                .into_iter()
                .map(|row| row.into_iter().map(|s| s.chars().next().unwrap()).collect())
                .collect()
            ),
            false
        );
    }
}
