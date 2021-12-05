use advent_2021::read_lines;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Unmarked(u8),
    Marked(u8),
}

type Board = Vec<Cell>;
type Calls = Vec<u8>;

const DIM: usize = 5;

#[test]
fn example_one() {
    let calls = vec![
        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
        26, 1,
    ];

    let boards: Vec<Board> = vec![
        vec![
            22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15,
            19,
        ]
        .into_iter()
        .map(|x| Cell::Unmarked(x))
        .collect(),
        vec![
            3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16, 12,
            6,
        ]
        .into_iter()
        .map(|x| Cell::Unmarked(x))
        .collect(),
        vec![
            14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3,
            7,
        ]
        .into_iter()
        .map(|x| Cell::Unmarked(x))
        .collect(),
    ];

    if let Some((winner, last_call)) = play(&calls, boards) {
        assert_eq!(4512, calculate_score(winner, last_call));
    } else {
        assert!(false);
    }
}

fn is_winner(board: &Board) -> bool {
    let row_win = board.chunks(DIM).any(|row| {
        row.iter().all(|cell| match cell {
            Cell::Unmarked(_) => false,
            Cell::Marked(_) => true,
        })
    });
    let col_win = (0..DIM)
        .map(|n| board.iter().skip(n).step_by(DIM))
        .any(|mut col| {
            col.all(|cell| match cell {
                Cell::Unmarked(_) => false,
                Cell::Marked(_) => true,
            })
        });
    row_win || col_win
}

fn play(calls: &Calls, mut boards: Vec<Board>) -> Option<(Board, u8)> {
    for call in calls {
        for board in boards.iter_mut() {
            for cell in board.iter_mut() {
                if let Cell::Unmarked(value) = cell {
                    if *value == *call {
                        *cell = Cell::Marked(*value);
                    }
                }
            }
            if is_winner(board) {
                return Some((board.clone(), *call));
            }
        }
    }
    None
}

fn calculate_score(board: Board, last_call: u8) -> u32 {
    let sum_of_unmarked = board.into_iter().fold(0u32, |sum, cell| match cell {
        Cell::Unmarked(n) => sum + n as u32,
        Cell::Marked(_) => sum,
    });
    sum_of_unmarked * last_call as u32
}

fn read_input_calls(filename: &str) -> Vec<u8> {
    if let Ok(lines) = read_lines(filename) {
        let mut calls = Vec::new();
        for line in lines.flatten() {
            let ns = line.split(',');
            for n in ns {
                if let Ok(call) = str::parse::<u8>(n) {
                    calls.push(call);
                }
            }
        }
        calls
    } else {
        Vec::new()
    }
}

fn read_input_boards(filename: &str) -> Vec<Board> {
    if let Ok(lines) = read_lines(filename) {
        let mut boards = Vec::new();
        let data: Vec<String> = lines.filter_map(|x| x.ok()).collect();
        let parse = |n| str::parse::<u8>(n).ok().map(Cell::Unmarked);
        for board in data.chunks(DIM + 1) {
            boards.push(
                vec![
                    board[0]
                        .split(char::is_whitespace)
                        .filter_map(parse)
                        .collect::<Vec<Cell>>(),
                    board[1]
                        .split(char::is_whitespace)
                        .filter_map(parse)
                        .collect::<Vec<Cell>>(),
                    board[2]
                        .split(char::is_whitespace)
                        .filter_map(parse)
                        .collect::<Vec<Cell>>(),
                    board[3]
                        .split(char::is_whitespace)
                        .filter_map(parse)
                        .collect::<Vec<Cell>>(),
                    board[4]
                        .split(char::is_whitespace)
                        .filter_map(parse)
                        .collect::<Vec<Cell>>(),
                ]
                .concat(),
            );
        }
        boards
    } else {
        Vec::new()
    }
}

fn is_same(first: &Board, second: &Board) -> bool {
    for (x, y) in first.iter().zip(second) {
        let x_value = match *x {
            Cell::Unmarked(n) => n,
            Cell::Marked(n) => n,
        };
        let y_value = match *y {
            Cell::Unmarked(n) => n,
            Cell::Marked(n) => n,
        };
        if x_value != y_value {
            return false;
        }
    }
    true
}

fn main() {
    let calls = read_input_calls("./input/day_4_calls.txt");
    let boards = read_input_boards("./input/day_4_boards.txt");

    // part 1
    {
        if let Some((winner, last_call)) = play(&calls, boards.clone()) {
            println!("Last call: {}, board: {:?}", last_call, winner);
            println!("{}", calculate_score(winner, last_call));
        } else {
            panic!();
        }
    }

    // part 2
    {
        // Squid's games:
        let mut remaining_boards = boards;
        while remaining_boards.len() > 1 {
            if let Some((winner, _)) = play(&calls, remaining_boards.clone()) {
                remaining_boards.retain(|board| !is_same(board, &winner));
            } else {
                panic!();
            }
        }

        // Play the final remaining board
        if let Some((winner, last_call)) = play(&calls, remaining_boards) {
            println!("Last call: {}, board: {:?}", last_call, winner);
            println!("{}", calculate_score(winner, last_call));
        } else {
            panic!();
        }
    }
}
