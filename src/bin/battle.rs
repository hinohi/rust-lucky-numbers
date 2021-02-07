use std::io::{stdin, stdout, BufRead, Write};

use lucky_numbers::{new_stack, Action, Board, Number};

enum UserInput {
    PopStack,
    Action(Action),
}

fn parse_pos(s: &str) -> Option<(usize, usize)> {
    let mut row = None;
    let mut col = None;
    for c in s.chars() {
        match c {
            '1'..='4' => row = Some((c as u8 - b'1') as usize),
            'a'..='d' => col = Some((c as u8 - b'a') as usize),
            'A'..='D' => col = Some((c as u8 - b'A') as usize),
            _ => return None,
        }
    }
    match (row, col) {
        (Some(row), Some(col)) => Some((row, col)),
        _ => None,
    }
}

fn parse_input(num: Option<Number>) -> UserInput {
    let stdin = stdin();
    let mut cin = stdin.lock();
    loop {
        match num {
            Some(num) => {
                println!("number: {}", num);
                print!("choice position (like 1A) or table: ");
                stdout().flush().unwrap();
                let mut s = String::new();
                cin.read_line(&mut s).unwrap();
                if s.trim() == "table" {
                    return UserInput::Action(Action::StackToTable(num));
                }
                if let Some((row, col)) = parse_pos(s.trim()) {
                    return UserInput::Action(Action::StackToSquare(row, col, num));
                }
            }
            None => {
                print!("pop stack or from table (like 10 1A): ");
                stdout().flush().unwrap();
                let mut s = String::new();
                cin.read_line(&mut s).unwrap();
                if s.trim() == "pop" || s.trim() == "stack" || s.trim() == "pop stack" {
                    return UserInput::PopStack;
                }
                let mut words = s.split_whitespace();
                let n = words.next().and_then(|s| s.parse().ok());
                let row_col = words.next().and_then(|s| parse_pos(s));
                match (n, row_col) {
                    (Some(n), Some((row, col))) => {
                        return UserInput::Action(Action::TableToSquare(row, col, n))
                    }
                    _ => (),
                }
            }
        }
    }
}

fn main() {
    let mut stack = new_stack(2);
    let mut board = Board::new(2, &mut stack);
    'GAME: while !stack.is_empty() {
        let counts = board.counts();
        if counts.iter().any(|c| *c == 16) {
            break;
        }
        println!("\n{}", board);
        let mut pop = None;
        loop {
            match parse_input(pop) {
                UserInput::PopStack => pop = stack.pop(),
                UserInput::Action(action) => match board.put(action) {
                    Ok(()) => continue 'GAME,
                    Err(()) => (),
                },
            }
        }
    }
    println!("{:?}", board.counts());
}
