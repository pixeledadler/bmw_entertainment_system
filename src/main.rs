use std::process::{Command, Stdio};
use std::io::{Error, Read};

fn now_playing() -> String {
    match Command::new("mpc").status() {
        Ok(_status) => {
            
            return _status.to_string();
        },
        Err(_err) => {
            return _err.to_string();
        }
    };
}

fn get_music_info(argument: String) -> String {
    let command = match Command::new("mpc")
        .arg("-f")
        .arg(argument)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
        Ok(command) => command,
        Err(_err) => return String::from("Failed")
    };
    let mut output = String::new();
    match command.stdout.unwrap().read_to_string(&mut output) {
        Err(_err) => throw_error(_err),
        Ok(_) => {
            let result = parse_mpc_output(output);
            return result;
        }
    };
    return String::from("upsii");
}

fn throw_error(err: Error) {
    println!("{:#?}", err);
}

fn parse_mpc_output(output: String) -> String {
    let mut lines = output.lines();
    let result = lines.next();
    return result.as_deref().unwrap().to_string();
}

fn main() {
    let artist_arg = String::from("%artist%");
    let title_arg = String::from("%title%");
    let track_arg = String::from("%track%");
    println!("{:#?}", get_music_info(artist_arg));
    println!("{}", get_music_info(title_arg));
}

