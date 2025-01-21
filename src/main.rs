use crossterm::{
    event::{self, KeyEvent, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use getopt::{Opt, Parser};
use std::{io::{self, Write}, string::{String, ToString}, vec::Vec};
use rand::prelude::*;
use std::fmt;

#[derive(Copy, Clone)]
enum Operator {
    Plus,
    Minus,
    Multi,
    Div
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

struct Bound {
    max: u32,
    min: u32,
    op: Operator 
}

fn operate(op: &Operator, x1: &i32, x2: &i32) -> i32 {
    match op {
        Operator::Plus => x1 + x2,
        Operator::Minus => x1 - x2,
        Operator::Multi => x1 * x2,
        Operator::Div => x1 / x2
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

fn generate_question(mn: u32, mx: u32, op: Operator) -> Question {
    // uniform sampling distribution within the bounds set
    // let mut uniform_sampler = rand::distributions::Uniform::new_inclusive(bound.min, bound.max);
    let mut rng = rand::thread_rng();
    let val1 = rng.gen_range(mn..mx) as i32;
    let val2 = rng.gen_range(mn..mx) as i32;
    let answer = operate(&op, &val1, &val2);
    return Question {
        val1,
        val2,
        answer,
        op
    }
} 

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Vec<String> = std::env::args().collect();
    let mut opts = Parser::new(&args, "ab:");

    let mut a_flag = false;
    let mut b_flag = String::new();
    loop {
        match opts.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('a', None) => a_flag = true,
                Opt('b', Some(string)) => b_flag = string.clone(),
                _ => unreachable!(),
            },
        }
    }

    let args = args.split_off(opts.index());
    println!("{}, {:?}", b_flag, a_flag);

    execute!(io::stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;

    // generate two numbers in a range
    //
    // return a struct of arg, operand, arg, answer
    // print args + operand to screen
    // take input with enter as submit
    // if correct move on, if wrong clear and reprint
    let q = generate_question(0, 20, Operator::Multi);

    write!(io::stdout(), "{} =", &q)?;
    io::stdout().flush()?;

    loop {
        match event::read()? {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Char(c) => { write!(io::stdout(), "{}", c.to_string())?; io::stdout().flush()?;  },
                    KeyCode::Enter => break,
                    _ => ()
                }
            },            
            _ => ()
        }
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}
