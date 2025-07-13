use crate::builtins::*;
use crate::parser::Command;

pub fn execute(command: Command) {
    match command {
        Command::Cat(args) => cat::cat(args),
        Command::Cd(path) => cd::cd(path),
        Command::Echo(args) => echo::echo(args),
        Command::Ls(args) => ls::ls(args),
        Command::Mkdir(args) => mkdir::mkdir(args),
        Command::Mv(args) => mv::mv(args),
        Command::Cp(args) => cp::cp(args),
        Command::Pwd => println!("{}",pwd::pwd()),
        Command::Rm(args, recursive) => rm::rm(args, recursive),
        _ => {}
    }
}
