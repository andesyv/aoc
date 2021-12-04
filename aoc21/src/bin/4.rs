type Board = Vec<Vec<(i32, bool)>>;

fn main() {
    const INPUT: &str = include_str!("../inputs/4.txt");
    // const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19\n\n 3 15  0  2 22\n 9 18 13 17  5\n19  8  7 25 23\n20 11 10 24  4\n14 21 16 12  6\n\n14 21 17 24  4\n10 16 15  9 19\n18  8 23 26 20\n22 11 13  6  5\n 2  0 12  3  7";

    let mut boards = INPUT.split("\n\n");
    let ran_numbers: Vec<i32> = boards
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut boards: Vec<Board> = boards
        .map(|b| {
            b.lines()
                .map(|l| {
                    l.split(' ')
                        .filter_map(|s| {
                            if let Ok(num) = s.parse::<i32>() {
                                Some((num, false))
                            } else {
                                None
                            }
                        })
                        .collect()
                })
                .collect()
        })
        .collect();
    let board_count = boards.len();
    let mut skipped_boards = Vec::new();

    for num in ran_numbers {
        for (i, board) in boards.iter_mut().enumerate() {
            if skipped_boards.contains(&i) {
                continue;
            }
            'outer: for row in board.iter_mut() {
                for (j, marked) in row {
                    if *j == num {
                        *marked = true;
                        break 'outer; // This is such a simple and smart feature
                    }
                }
            }
            if has_won(board) {
                let unmarked_sum: i32 = board
                    .concat()
                    .iter()
                    .filter_map(|(i, marked)| if !marked { Some(i) } else { None })
                    .sum();
                println!(
                    "Board {} won{} with an unmarked sum of {} and a score of {}",
                    i + 1,
                    if skipped_boards.is_empty() {
                        " first"
                    } else if board_count - 1 == skipped_boards.len() {
                        " last"
                    } else {
                        ""
                    },
                    unmarked_sum,
                    unmarked_sum * num
                );
                skipped_boards.push(i);
            }
        }
    }
}

// https://www.hackertouch.com/matrix-transposition-in-rust.html
fn matrix_transpose<T: Copy>(m: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut t = vec![Vec::with_capacity(m.len()); m[0].len()];
    for r in m {
        for i in 0..r.len() {
            t[i].push(r[i]);
        }
    }
    t
}

fn has_won(board: &Board) -> bool {
    let check_rows = |b: &Board| b.iter().any(|row| row.iter().all(|(_, marked)| *marked));
    check_rows(board) || check_rows(&matrix_transpose(board))
}
