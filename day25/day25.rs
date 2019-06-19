use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Copy, Clone)]
struct Action {
    write: usize,
    move_right: bool,
    next_state: u8,
}

impl Action {
    fn new() -> Self {
        Action {
            write: 0,
            move_right: false,
            next_state: 0,
        }
    }
}

fn main() {
    let mut lines = BufReader::new(io::stdin()).lines();

    let initial_state = lines.next().unwrap().unwrap();
    let initial_state = initial_state.as_bytes()[initial_state.len() - 2];

    let steps = lines.next().unwrap().unwrap();
    let steps: u32 = steps
        .split(' ')
        .skip_while(|word| word.as_bytes()[0].is_ascii_alphabetic())
        .next()
        .unwrap()
        .parse()
        .unwrap();

    let mut state_actions = HashMap::new();

    while let Some(_) = lines.next() {
        let state = lines.next().unwrap().unwrap();
        let state = state.as_bytes()[state.len() - 2];
        let mut actions = [Action::new(); 2];

        for _ in 0..2 {
            let value = lines.next().unwrap().unwrap();
            let value = (value.as_bytes()[value.len() - 2] - b'0') as usize;

            let write = lines.next().unwrap().unwrap();
            let write = (write.as_bytes()[write.len() - 2] - b'0') as usize;

            let move_right = lines.next().unwrap().unwrap();
            let move_right = move_right.split(' ').last().unwrap() == "right.";

            let next_state = lines.next().unwrap().unwrap();
            let next_state = next_state.as_bytes()[next_state.len() - 2];

            actions[value] = Action {
                write,
                move_right,
                next_state,
            };
        }

        state_actions.insert(state, actions);
    }

    let mut state = initial_state;
    let mut cursor = 0;
    let mut tape: HashMap<i32, usize> = HashMap::new();

    for _ in 0..steps {
        let &value = tape.get(&cursor).unwrap_or(&0);
        let action = state_actions.get(&state).unwrap()[value];

        tape.insert(cursor, action.write);
        cursor += if action.move_right { 1 } else { -1 };
        state = action.next_state;
    }

    println!("part 1:  {}", tape.values().filter(|&&v| v == 1).count());
}
