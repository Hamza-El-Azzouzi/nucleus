use crate::builtins::*;
use crate::parser::Command;

pub fn execute(command: Command) {
    match command {
        Command::Cat(vec) => cat::cat(vec),
        Command::Cd(op) => cd::cd(op),
        Command::Echo(vec) => echo::echo(vec),
        Command::Ls(vec, op) => ls::ls(vec, op),
        Command::Mkdir(vec) => mkdir::mkdir(vec),
        Command::Mv(vec) => mv::mv(vec),
        Command::Pwd => pwd::pwd(),
        Command::Rm(vec, recursive) => rm::rm(vec, recursive),
        _ => {}
    }
}
