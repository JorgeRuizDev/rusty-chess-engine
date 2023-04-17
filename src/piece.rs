use std::{fmt, rc::Rc};

use crate::moves::diag::Diagonal;
use crate::moves::line::Line;
use crate::{board::Coord, moves::Move};
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Black => write!(f, "B"),
            Self::White => write!(f, "W"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]

pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Bishop => write!(f, "♝"),
            Self::King => write!(f, "♚"),
            Self::Queen => write!(f, "♛"),
            Self::Rook => write!(f, "♜"),
            Self::Knight => write!(f, "♞"),
            Self::Pawn => write!(f, "♟︎"),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Piece {
    pub color: Color,
    pub piece: PieceType,
    pub has_moved: bool,
    // Mutable Cell reference:
    pub coord: Coord,
    pub moves: Vec<Rc<dyn Move>>,
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.color, self.piece)
    }
}

impl Piece {
    pub fn new(color: Color, piece: PieceType, moves: Vec<Rc<dyn Move>>, coord: Coord) -> Self {
        Self {
            color,
            piece,
            has_moved: false,
            moves: moves,
            coord,
        }
    }

    pub fn new_rook(color: Color, coord: Coord) -> Self {
        Self::new(
            color,
            PieceType::Rook,
            vec![Rc::new(Line::new(None))],
            coord,
        )
    }

    pub fn new_bishop(color: Color, coord: Coord) -> Self {
        Self::new(
            color,
            PieceType::Bishop,
            vec![Rc::new(Diagonal::new(None))],
            coord,
        )
    }

    pub fn new_queen(color: Color, coord: Coord) -> Self {
        Self::new(
            color,
            PieceType::Queen,
            vec![Rc::new(Line::new(None)), Rc::new(Diagonal::new(None))],
            coord,
        )
    }

    pub fn new_king(color: Color, coord: Coord) -> Self {
        // TODO: Add Castle Move
        Self::new(
            color,
            PieceType::King,
            vec![Rc::new(Line::new(Some(1))), Rc::new(Diagonal::new(Some(1)))],
            coord,
        )
    }

    pub fn new_pawn(color: Color, coord: Coord) -> Self {
        // TODO: Add En Passant + Default move
        Self::new(color, PieceType::Pawn, vec![], coord)
    }

    pub fn new_knight(color: Color, coord: Coord) -> Self {
        // TODO: Add jump move
        Self::new(color, PieceType::Knight, vec![], coord)
    }
}

#[cfg(test)]
mod piece_tests {
    use crate::board::Coord;

    use super::*;
}
