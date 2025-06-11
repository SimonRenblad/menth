use crossterm::{
    cursor::{self, MoveTo},
    event::{self, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, is_raw_mode_enabled, Clear, ClearType,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use getopt::{Opt, Parser};
use rand::prelude::*;
use std::fmt;
use std::println;
use std::{
    io::{self, Write},
    string::{String, ToString},
    vec::Vec,
};

#[derive(Copy, Clone)]
enum Operator {
    Plus,
    Minus,
    Multi,
    Div,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Multi => write!(f, "*"),
            Operator::Div => write!(f, "/"),
        }
    }
}

impl From<char> for Operator {
    fn from(c: char) -> Operator {
        match c {
            '+' => Operator::Plus,
            '-' => Operator::Minus,
            '*' => Operator::Multi,
            '/' => Operator::Div,
            c => panic!("char '{}' cannot be converted to Operator", c),
        }
    }
}

fn operate(op: &Operator, x1: &i32, x2: &i32) -> i32 {
    match op {
        Operator::Plus => x1 + x2,
        Operator::Minus => x1 - x2,
        Operator::Multi => x1 * x2,
        Operator::Div => x1 / x2,
    }
}

struct Question {
    val1: i32,
    val2: i32,
    answer: i32,
    op: Operator,
}

impl fmt::Display for Question {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", &self.val1, &self.op, &self.val2)
    }
}

fn generate_question(mn: u32, mx: u32, allowed_ops: &Vec<Operator>) -> Question {
    // uniform sampling distribution within the bounds set
    // let mut uniform_sampler = rand::distributions::Uniform::new_inclusive(bound.min, bound.max);
    let mut rng = rand::thread_rng();
    let val1 = rng.gen_range(mn..mx) as i32;
    let val2 = rng.gen_range(mn..mx) as i32;
    let l = allowed_ops.len();
    let i = rng.gen_range(0..l);
    let op = allowed_ops[i];
    let answer = operate(&op, &val1, &val2);
    return Question {
        val1,
        val2,
        answer,
        op,
    };
}

enum State {
    Incorrect,
    Answer,
    Ask,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match run() {
        Ok(()) => Ok(()),
        Err(e) => {
            if is_raw_mode_enabled()? {
                disable_raw_mode()?;
            }
            execute!(io::stdout(), LeaveAlternateScreen)?;
            Err(e)
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = Parser::new(&args, "hp:r:");

    execute!(io::stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut operators = String::new();
    let mut bounds = String::new();
    loop {
        match opts.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('h', None) => println!("menth - mental math trainer"),
                Opt('p', Some(s)) => operators = s.clone(),
                Opt('r', Some(s)) => bounds = s.clone(),
                _ => unreachable!(),
            },
        }
    }

    let mut allowed_ops: Vec<Operator> = Vec::new();
    // add choices for operators
    for c in operators.chars() {
        allowed_ops.push(c.into());
    }

    // set ranges
    let split: Vec<&str> = bounds.split(",").collect();
    let mn = split.get(0).unwrap().parse::<u32>()?;
    let mx = split.get(1).unwrap().parse::<u32>()? + 1;

    execute!(io::stdout(), MoveTo(1, 1))?;

    let mut answer_buffer = String::with_capacity(32); // answers are short
    let mut current_state = State::Ask;
    let mut current_question = Question {
        val1: 0,
        val2: 0,
        answer: 0,
        op: Operator::Plus,
    };
    loop {
        match current_state {
            State::Ask => {
                execute!(io::stdout(), MoveTo(1, 1))?;
                execute!(io::stdout(), Clear(ClearType::CurrentLine))?;
                current_question = generate_question(mn, mx, &allowed_ops);
                write!(io::stdout(), "{} = ", &current_question)?;
                io::stdout().flush()?;
                answer_buffer.clear();
                current_state = State::Answer;
            }
            State::Incorrect => {
                execute!(io::stdout(), MoveTo(1, 1))?;
                execute!(io::stdout(), Clear(ClearType::CurrentLine))?;
                write!(io::stdout(), "Incorrect! Answer: {}", &current_question.answer)?;
                io::stdout().flush()?;
                current_state = State::Ask;
            }
            State::Answer => (),
        }
        match event::read()? {
            Event::Key(event) => match event.code {
                KeyCode::Char(c) => {
                    if matches!(current_state, State::Answer) {
                        write!(io::stdout(), "{}", c.to_string())?;
                        io::stdout().flush()?;
                        answer_buffer.push(c);
                    }
                }
                KeyCode::Enter => {
                    if matches!(current_state, State::Answer) {
                        current_state = match answer_buffer.parse::<i32>() {
                            Ok(ans) if ans == current_question.answer => State::Ask,
                            _ => State::Incorrect,
                        }
                    }
                }
                KeyCode::Backspace | KeyCode::Delete => {
                    if matches!(current_state, State::Answer) {
                        if let Some(_) = answer_buffer.pop() {
                            execute!(io::stdout(), cursor::MoveLeft(1))?;
                            execute!(io::stdout(), Clear(ClearType::UntilNewLine))?;
                        }
                    }
                }
                KeyCode::Esc => break,
                _ => (),
            },
            _ => (),
        }
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}
