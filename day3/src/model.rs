pub enum Operation {
    Mul,
    Do,
    Dont,
}

pub enum State {
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

pub struct Program {
    pub state: State,
    pub left: String,
    pub enable_control_flow: bool,
    pub enable: bool,
    pub right: String,
}

impl Program {
    pub fn new(enable_control_flow: bool) -> Self {
        Self {
            left: String::new(),
            enable_control_flow,
            right: String::new(),
            state: State::Start,
            enable: true,
        }
    }

    pub fn clear(&mut self) {
        self.left = String::new();
        self.right = String::new();
        self.state = State::Start;
    }
}
