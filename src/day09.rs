#[derive(Debug, Copy, Clone)]
struct State {
    group_depth: u32,
    garbage: GarbageState,
    cancel_next: CancelState,
}

#[derive(Debug, Copy, Clone)]
enum GarbageState {
    Garbage,
    None,
}

#[derive(Debug, Copy, Clone)]
enum CancelState {
    Cancel,
    None,
}

impl State {
    pub fn new() -> Self {
        Self {
            group_depth: 0,
            garbage: GarbageState::None,
            cancel_next: CancelState::None,
        }
    }
}

/// The score of the program and the number of garbage characters
type ProgramOutput = (u32, u32);

pub fn process_program(input: &str) -> ProgramOutput {
    use CancelState as C;
    use GarbageState as G;
    let mut state = State::new();
    let mut score = 0;
    let mut garbage_count = 0;

    for char in input.chars() {
        // match order is important for operator precedence
        match (state.group_depth, state.garbage, state.cancel_next, char) {
            (_, _, C::Cancel, _) => state.cancel_next = C::None,
            (_, _, C::None, '!') => state.cancel_next = C::Cancel,
            (_, G::Garbage, _, '>') => state.garbage = G::None,
            (_, G::Garbage, _, _) => garbage_count += 1,
            (_, G::None, _, '<') => state.garbage = G::Garbage,
            (_, _, _, '{') => state.group_depth += 1,
            (_, _, _, ',') => {}
            (depth, _, _, '}') => {
                score += depth;
                if depth > 0 {
                    state.group_depth -= 1;
                }
            }
            (_, _, _, c) => {
                dbg!(state);
                dbg!(c);
                panic!("Unhandled state!");
            }
        }
    }

    (score, garbage_count)
}

#[cfg(test)]
mod tests {
    use super::process_program;

    #[test]
    fn process_program_score_works() {
        let input = "{{<ab>},{<ab>},{<ab>},{<ab>}}";
        let score = process_program(input).0;
        assert_eq!(9, score);

        let input = "{{<a!>},{<a!>},{<a!>},{<ab>}}";
        let score = process_program(input).0;
        assert_eq!(3, score);
    }

    #[test]
    fn process_program_non_garbage_characters_works() {
        let input = "<{o\"i!a,<{i<a>";
        let garbage_chars = process_program(input).1;

        assert_eq!(10, garbage_chars);

        let input = "<random characters>";
        let garbage_chars = process_program(input).1;

        assert_eq!(17, garbage_chars);
    }
}
