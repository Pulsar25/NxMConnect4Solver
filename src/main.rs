use std::collections::HashSet;
use std::fs::OpenOptions;
use std::hash::Hash;
use std::io::{Write, BufWriter,BufRead, BufReader};
use std::sync::{Arc, Mutex};
extern crate crossbeam;
use crossbeam::channel::unbounded;
use std::fs::File;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Disc {
    Empty,
    Red,
    Yellow,
}

struct Connect4 {
    board: Vec<Vec<Disc>>,
    rows: usize,
    columns: usize,
    current_player: Disc,
    winner: Option<Disc>,
}

impl Connect4 {
    fn new(rows: usize, columns: usize) -> Connect4 {
        let board = vec![vec![Disc::Empty; columns]; rows];
        Connect4 {
            board,
            rows,
            columns,
            current_player: Disc::Red,
            winner: None,
        }
    }

    fn drop_disc(&mut self, column: usize) -> Result<(), &'static str> {
        if self.winner.is_some() {
            return Err("Game over. Please start a new game.");
        }

        if column >= self.columns {
            return Err("Invalid column.");
        }

        for row in (0..self.rows).rev() {
            if self.board[row][column] == Disc::Empty {
                self.board[row][column] = self.current_player;

                if self.check_winner(row, column) {
                    self.winner = Some(self.current_player);
                }

                self.current_player = match self.current_player {
                    Disc::Red => Disc::Yellow,
                    Disc::Yellow => Disc::Red,
                    Disc::Empty => Disc::Empty,
                };
                return Ok(());
            }
        }

        Err("Column is full.")
    }

    fn check_winner(&self, row: usize, column: usize) -> bool {
        let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (1, 1), (-1, 1)];

        for &(dr, dc) in &directions {
            let mut count = 1;
            for dir in [-1, 1] {
                for mult in 0..3 {
                    let r = row as i32 + dir * mult * dr;
                    let c = column as i32 + dir * mult * dc;
                    if r < 0 || r >= self.rows as i32 || c < 0 || c >= self.columns as i32 {
                        break;
                    }
                    if self.board[r as usize][c as usize] != self.current_player {
                        break;
                    }
                    count += 1;
                }
                if count == 4 {
                    return true;
                }
            }
        }
        false
    }
}

fn solve_NxM(rows: usize, columns: usize) {
    
}
