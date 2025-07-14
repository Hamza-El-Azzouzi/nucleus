use crate::prelude::*;

pub enum Color {
    Red,
    Orange,
    Green,
    Blue,
    SkyBlue,
    Brown,
    Pink,
}

pub fn colorize(text: &str, color: Color, bold: bool) -> String {
    let result = match color {
        Color::Red => text.red(),
        Color::Green => text.green(),
        Color::Blue => text.blue(),
        Color::Orange => text.truecolor(255, 165, 0),
        Color::SkyBlue => text.truecolor(45, 158, 179),
        Color::Brown => text.truecolor(156, 109, 71),
        Color::Pink => text.truecolor(162, 71, 185),
    };

    if bold {
        result.bold().to_string()
    } else {
        result.to_string()
    }
}

pub fn colorize_device(file_name: &mut String, _flags: &Flag) {
    let styled = colorize(file_name, Color::Brown, true);
    *file_name = styled.on_black().to_string();
}

pub fn colorize_dir(file_name: &mut String, flags: &Flag) {
    *file_name = colorize(file_name, Color::Blue, true);
    if flags.f {
        file_name.push('/');
    }
}

pub fn colorize_executable(file_name: &mut String, flags: &Flag) {
    *file_name = colorize(file_name, Color::Green, true);
    if flags.f {
        file_name.push('*');
    }
}

pub fn colorize_pipe(file_name: &mut String, flags: &Flag) {
    *file_name = colorize(file_name, Color::Brown, false)
        .on_black()
        .to_string();
    if flags.f {
        file_name.push('|');
    }
}

pub fn colorize_socket(file_name: &mut String, flags: &Flag) {
    *file_name = colorize(file_name, Color::Pink, false);
    if flags.f {
        file_name.push('|');
    }
}

pub fn colorize_symlink(file_name: &mut String, is_broken: bool, flags: &Flag) {
    let color = if is_broken {
        Color::Red
    } else {
        Color::SkyBlue
    };
    *file_name = colorize(file_name, color, true);

    if flags.f && !flags.l {
        file_name.push('@');
    }
}
