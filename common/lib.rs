use std::{io::{self, Read}, fs::File, path::Path, fmt::{Debug, Display}};

#[macro_export]
macro_rules! get_input {
    () => {
        $crate::read_input(std::path::Path::new(&format!("inputs/{}.txt", module_path!())))
    };
    (lines) => {
        $crate::get_input!().lines()
    };
    (parsed) => {
        $crate::get_input!(lines).map(|line| line.parse().expect("invalid parse"))
    };
}

/// shrlorp an input path or read from stdin if it doesn't exist
pub fn read_input(path: &Path) -> String {
    match File::open(path) {
        Ok(mut f) => {
            let mut s = String::new();
            f.read_to_string(&mut s).expect("could not read file");
            s
        },
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            println!("input file {} not found - enter your input and press Ctrl-D when done", path.display());
            let mut s = String::new();
            io::stdin().lock().read_to_string(&mut s).expect("could not read stdin");
            s
        },
        Err(e) => panic!("Unknown error reading input: {:?}", e),
    }
}

pub struct PanicOnError;

impl Debug for PanicOnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("call From<T> to use correctly")
    }
}

impl<T: Display> From<T> for PanicOnError {
    fn from(error: T) -> PanicOnError { panic!("error: {}", error) }
}
