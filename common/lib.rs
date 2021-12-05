use std::{
    fmt::{self, Debug, Display, Write},
    fs::File,
    io::{self, Read},
    path::Path,
};

use ndarray::Array2;

#[macro_export]
macro_rules! get_input {
    () => {
        $crate::read_input(std::path::Path::new(&format!(
            "inputs/{}.txt",
            module_path!()
        )))
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
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            println!(
                "input file {} not found - enter your input and press Ctrl-D when done",
                path.display()
            );
            let mut s = String::new();
            io::stdin()
                .lock()
                .read_to_string(&mut s)
                .expect("could not read stdin");
            s
        }
        Err(e) => panic!("Unknown error reading input: {:?}", e),
    }
}

pub struct PanicOnError;

impl Debug for PanicOnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("call From<T> to use correctly")
    }
}

impl<T: Display> From<T> for PanicOnError {
    fn from(error: T) -> PanicOnError {
        panic!("error: {}", error)
    }
}

pub struct Dots<'a, T>(pub &'a Array2<T>);
impl<'a, T> Display for Dots<'a, T>
where
    T: Display + num_traits::Zero,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.0.rows() {
            for e in row {
                if e.is_zero() {
                    f.write_char('.')?;
                } else {
                    Display::fmt(e, f)?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
