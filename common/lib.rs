use std::{io::{BufReader, BufRead}, fs::File, path::Path};

#[macro_export]
macro_rules! get_input {
    () => {
        $crate::read_input(std::path::Path::new(&format!("inputs/{}.txt", module_path!())))
    };
    (lines) => {
        {
            use std::io::BufRead;
            $crate::get_input!().lines().map(|line| line.expect("invalid line"))
        }
    };
    (parsed) => {
        $crate::get_input!(lines).map(|line| line.parse().expect("invalid parse"))
    };
}

pub fn read_input(path: &Path) -> impl BufRead {
    let f = File::open(path).expect("cannot open input");
    BufReader::new(f)
}
