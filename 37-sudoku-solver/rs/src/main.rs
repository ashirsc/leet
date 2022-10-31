use serde::{Deserialize, Serialize};
use serde_json::json;

fn main() {
    let res: SodukuApiRes =
        match reqwest::blocking::get("https://sugoku.herokuapp.com/board?difficulty=easy") {
            Err(err) => panic!("Failed to get board: {}", err),
            Ok(res) => {
                let body = res.text().expect("Couldn't convert response body to text");
                // println!("{}", body);
                serde_json::from_str(&body).unwrap()
            }
        };

    // let res:SodukuApiRes = serde_json::from_value(
    //     json!({"board":[[0,0,2,0,0,0,0,0,8],[0,3,0,0,8,0,0,6,7],[0,0,9,1,5,0,0,3,0],[0,1,0,0,0,0,7,0,9],[0,0,6,8,9,7,1,2,0],[8,0,0,0,0,2,4,5,0],[3,0,0,9,4,8,0,0,0],[0,4,0,0,7,0,0,9,2],[0,7,5,0,0,3,8,0,0]]}),
    // ).unwrap();

    let mut board = Vec::new();

    for uRow in res.board {
        let mut cRow = Vec::new();
        for val in uRow {
            let c = match val {
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                7 => '7',
                8 => '8',
                9 => '9',
                _ => '.',
            };
            cRow.push(c);
        }
        board.push(cRow)
    }


    Solution::solve_sudoku(&mut board);
}

#[derive(Serialize, Deserialize, Debug)]
struct SodukuApiRes {
    board: Vec<Vec<u8>>,
}

struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut iterations = 0;
        let mut edit_count = -1;
        let mut board_completed = is_complete(board);

        println!("Starting board");
        print_board(board);


        while !board_completed && edit_count != 0 {
            edit_count = 0;


            // at each box
            for i in 0..=8 {
                for j in 0..=8 {
                    if board[i][j] != '.' {
                        continue;
                    }

                    // find possible values
                    let mut options: Vec<char> = vec![];
                    for check_val in '1'..='9' {
                        let row_check = row_has(check_val, i, board);
                        let col_check = col_has(check_val, j, board);
                        let sq_check = sq_has(check_val, i, j, board);

                        if !row_check && !col_check && !sq_check {
                            options.push(check_val)
                        }
                    }
                    // if only one possible fill in board
                    if options.len() == 1 {
                        board[i][j] = *options.first().unwrap();
                        edit_count += 1
                    }
                }
            }
            
            iterations += 1;
            board_completed = is_complete(board);
        }

       
        if board_completed {
            println!("Completed in {} iterations", iterations)
        } else {
            println!("Wasn't able to make any edits after {} iterations", iterations)
        }
        print_board(board);


    }
}

fn row_has(val: char, i: usize, board: &mut Vec<Vec<char>>) -> bool {
    for v in board.get(i).unwrap().iter() {
        if *v == val {
            return true;
        }
    }
    false
}
fn col_has(val: char, j: usize, board: &mut Vec<Vec<char>>) -> bool {
    for row in board {
        if *row.get(j).unwrap() == val {
            return true;
        }
    }
    false
}
fn sq_has(val: char, i: usize, j: usize, board: &mut Vec<Vec<char>>) -> bool {
    let sqi = (i / 3) * 3;
    let sqj = (j / 3) * 3;

    for k in sqi..sqi + 3 {
        for l in sqj..sqj + 3 {
            if *board.get(k).unwrap().get(l).unwrap() == val {
                return true;
            }
        }
    }
    false
}

fn is_complete(board: &mut Vec<Vec<char>>) -> bool {
    for i in 0..=8 {
        for j in 0..=8 {
            if *board.get(i).unwrap().get(j).unwrap() == '.' {
                return false;
            }
        }
    }
    true
}

fn print_board(board: &mut Vec<Vec<char>>) {
    for (i, row) in board.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if j > 0 && j % 3 == 0 {
                print!("| ")
            }
            print!("{} ", val)
        }
        println!();
        if i < 8 && (i + 1) % 3 == 0 {
            println!("---------------------");
        }
    }
    println!("\n");
}
