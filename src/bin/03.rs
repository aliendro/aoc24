use std::fs;

fn main() {
    let content = fs::read_to_string("data/inputs/03").expect("Failed to read from input");

    let part01_solution = solve(&content, false);

    println!("Part1: {part01_solution}");

    let part02_solution = solve(&content, true);

    println!("Part2: {part02_solution}");
}

enum Operation {
    Mul,
    Do,
    Dont,
}

enum State {
    Start,
    ReadingM,
    ReadingU,
    ReadingL,
    ReadingOpenParen(Operation),
    ReadingFirstOperand,
    ReadingComma,
    ReadingSecondOperand,
    ReadingD,
    ReadingO,
    ReadingN,
    ReadingApos,
    ReadingT,
}

struct Program {
    state: State,
    left: String,
    enable_control_flow: bool,
    enable: bool,
    right: String,
}

impl Program {
    fn new(enable_control_flow: bool) -> Self {
        Self {
            left: String::new(),
            enable_control_flow,
            right: String::new(),
            state: State::Start,
            enable: true,
        }
    }

    fn clear(&mut self) {
        self.left = String::new();
        self.right = String::new();
        self.state = State::Start;
    }
}

fn solve(text: &str, enable_control_flow: bool) -> u32 {
    let mut program = Program::new(enable_control_flow);
    let mut result = 0;

    for char in text.chars() {
        match (&program.state, char) {
            (State::Start, 'm') => {
                program.state = State::ReadingM;
            }
            (State::ReadingM, 'u') => {
                program.state = State::ReadingU;
            }
            (State::ReadingU, 'l') => {
                program.state = State::ReadingL;
            }
            (State::ReadingL, '(') => {
                program.state = State::ReadingOpenParen(Operation::Mul);
            }
            (State::ReadingOpenParen(Operation::Mul) | State::ReadingFirstOperand, '0'..='9') => {
                program.left.push(char);
                program.state = State::ReadingFirstOperand;
            }
            (State::ReadingFirstOperand, ',') => {
                program.state = State::ReadingComma;
            }
            (State::ReadingComma | State::ReadingSecondOperand, '0'..='9') => {
                program.right.push(char);
                program.state = State::ReadingSecondOperand;
            }
            (State::ReadingSecondOperand, ')') => {
                if !program.enable {
                    continue;
                }

                let (left, right) = (
                    program
                        .left
                        .parse::<u32>()
                        .expect("Failed to parse left operand"),
                    program
                        .right
                        .parse::<u32>()
                        .expect("Failed to parse right operand"),
                );

                result += left * right;

                program.clear();
            }
            (State::Start, 'd') => {
                program.state = State::ReadingD;
            }
            (State::ReadingD, 'o') => {
                program.state = State::ReadingO;
            }
            (State::ReadingO, '(') => {
                program.state = State::ReadingOpenParen(Operation::Do);
            }
            (State::ReadingOpenParen(Operation::Do), ')') => {
                program.enable = true;
                program.clear();
            }
            (State::ReadingO, 'n') => {
                program.state = State::ReadingN;
            }
            (State::ReadingN, '\'') => {
                program.state = State::ReadingApos;
            }
            (State::ReadingApos, 't') => {
                program.state = State::ReadingT;
            }
            (State::ReadingT, '(') => {
                program.state = State::ReadingOpenParen(Operation::Dont);
            }
            (State::ReadingOpenParen(Operation::Dont), ')') => {
                if program.enable_control_flow {
                    program.enable = false;
                }
                program.clear();
            }

            _ => {
                program.clear();
            }
        }
    }

    result
}
