use crate::parser;

pub fn shell(cmd: parser::Cmd) -> String {
    let command: String = cmd.name.unwrap() + " " + &cmd.args.unwrap().join(" ");
    command.to_string()
}
