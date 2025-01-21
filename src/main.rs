use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use getopt::{Opt, Parser};
use std::{io, string::String, vec::Vec};

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

    loop {
        if matches!(event::read()?, Event::Key(_)) {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}
