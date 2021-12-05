// This almost certainly would've been easier with string manipulation
// instead of the bit twiddling I did.

use std::{collections::HashMap, iter};

use common::get_input;
use ndarray::{Array, Array2, ArrayView, ArrayView2, ArrayViewMut2, Zip};

#[derive(Clone, Debug)]
struct Input {
    drawings: Vec<i32>,
    boards: Vec<Board>,
}

#[derive(Clone, Debug)]
struct Board(Array2<i32>);

fn parse_input<'a>(mut iter: impl Iterator<Item = &'a str>) -> Input {
    let drawings = iter
        .next()
        .unwrap()
        .split(",")
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut boards = vec![];
    while let Some(empty) = iter.next() {
        let mut board = Array::zeros((0, 5));

        assert!(empty.is_empty());
        for _ in 0..5 {
            let row = iter
                .next()
                .expect("missing bingo row")
                .split_ascii_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<i32>, _>>()
                .expect("broken bingo");
            assert_eq!(row.len(), 5);
            board.push_row(ArrayView::from(&row)).unwrap();
        }
        // assert_eq!(rows.len(), 5);
        boards.push(Board(board));
    }
    Input { drawings, boards }
}

// mark a piece on the board and return if that made bingo
fn mark_board(mut a: ArrayViewMut2<bool>, (row, col): (usize, usize)) -> bool {
    a[(row, col)] = true;
    a.row(row).into_iter().all(|&x| x) || a.column(col).into_iter().all(|&x| x)
}

/// Returns the sum of all unmarked pieces
fn unmarked_score(pieces: ArrayView2<i32>, marks: ArrayView2<bool>) -> i32 {
    Zip::from(&pieces)
        .and(&marks)
        .fold(0, |acc, &piece, &marked| acc + piece * (!marked as i32))
}

fn calc_positions(boards: &[Board]) -> HashMap<i32, Vec<(usize, (usize, usize))>> {
    let mut positions: HashMap<i32, Vec<(usize, (usize, usize))>> = HashMap::new();
    for (idx, board) in boards.iter().enumerate() {
        for (pos, &ball) in board.0.indexed_iter() {
            positions.entry(ball).or_default().push((idx, pos));
        }
    }
    positions
}

fn part1(input: &Input) -> i32 {
    let mut marks: Vec<Array2<bool>> = input
        .boards
        .iter()
        .map(|_| Array::default((5, 5)))
        .collect();
    let positions = calc_positions(&input.boards);

    // make marks on board
    for drawing in &input.drawings {
        if let Some(p) = positions.get(drawing) {
            for (idx, pos) in p {
                if mark_board(ArrayViewMut2::from(&mut marks[*idx]), *pos) {
                    println!("bingo!\n{:?}", marks[*idx]);
                    return unmarked_score(
                        ArrayView::from(&input.boards[*idx].0),
                        ArrayView::from(&marks[*idx]),
                    ) * *drawing;
                }
            }
        }
    }
    panic!("no win after drawings!")
}

fn part2(input: &Input) -> i32 {
    let mut marks: Vec<Array2<bool>> = input
        .boards
        .iter()
        .map(|_| Array::default((5, 5)))
        .collect();
    let positions = calc_positions(&input.boards);
    let mut got_bingo: Vec<bool> = iter::repeat(false).take(marks.len()).collect();
    let mut num_bingos = 0;

    // there are definitely more algorithmically kind ways to do this
    // bounded input is bounded input
    for drawing in &input.drawings {
        if let Some(p) = positions.get(drawing) {
            for (idx, pos) in p {
                let idx = *idx;
                let pos = *pos;
                if got_bingo[idx] {
                    continue;
                }
                if mark_board(ArrayViewMut2::from(&mut marks[idx]), pos) {
                    num_bingos += 1;
                    if num_bingos == input.boards.len() {
                        println!(
                            "last bingo (win {}, idx {})!\n{:?}\n{:?}",
                            num_bingos, idx, input.boards[idx].0, marks[idx]
                        );
                        return unmarked_score(
                            ArrayView::from(&input.boards[idx].0),
                            ArrayView::from(&marks[idx]),
                        ) * *drawing;
                    }
                    got_bingo[idx] = true;
                }
            }
        }
    }
    panic!("no last bingo?");
}

fn main() {
    let input = parse_input(get_input!(lines));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
