use crate::prelude::*;

pub struct Flag {
    pub l: bool,
    pub a: bool,
    pub f: bool,
}

impl Flag {
    pub fn parse(
        args: &Vec<String>,
        directories: &mut Vec<PathBuf>,
        files: &mut Vec<PathBuf>,
    ) -> Result<Self, String> {
        let mut flags = Self {
            a: false,
            f: false,
            l: false,
        };
        for arg in args {
            if arg.starts_with('-') {
                for ch in arg.chars().skip(1) {
                    match ch {
                        'a' => flags.a = true,
                        'F' => flags.f = true,
                        'l' => flags.l = true,
                        _ => {
                            return Err(format!(
                                "invalid flag: '{ch}', supported flags are: '-a', '-F', '-l'"
                            ));
                        }
                    }
                }
            } else {
                let path = PathBuf::from(arg);
                if path.is_dir() {
                    directories.push(path);
                } else if path.symlink_metadata().is_ok() {
                    files.push(path);
                } else {
                    return Err(format!("cannot access {arg:?}: No such file or directory"));
                }
            }
        }

        Ok(flags)
    }
}
