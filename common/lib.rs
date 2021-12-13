#![warn(unsafe_op_in_unsafe_fn)]

use std::{
    fmt::{self, Debug, Display, Write},
    fs::File,
    io::{self, Read},
    mem::MaybeUninit,
    panic::{self, AssertUnwindSafe},
    path::Path,
    ptr,
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

pub trait ArrayCollect<T> {
    fn array_collect<const N: usize>(self) -> Option<[T; N]>;
}

// adapted from stdlib
unsafe fn array_assume_init<T, const N: usize>(array: [MaybeUninit<T>; N]) -> [T; N] {
    // SAFETY:
    // * The caller guarantees that all elements of the array are initialized
    // * `MaybeUninit<T>` and T are guaranteed to have the same layout
    // * `MaybeUninit` does not drop, so there are no double-frees
    // And thus the conversion is safe
    unsafe {
        // intrinsics::assert_inhabited::<[T; N]>();
        (&array as *const _ as *const [T; N]).read()
    }
}

impl<I: Iterator> ArrayCollect<I::Item> for I {
    fn array_collect<const N: usize>(mut self) -> Option<[I::Item; N]> {
        // yes i promise this is a sound way to do this
        // https://doc.rust-lang.org/nomicon/unchecked-uninit.html
        let mut array: [MaybeUninit<I::Item>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        let mut written_indexes: usize = 0;
        match panic::catch_unwind(AssertUnwindSafe(|| {
            for dst in &mut array {
                dst.write(self.next()?);
                written_indexes = written_indexes.wrapping_add(1);
            }
            Some(())
        })) {
            Ok(x) => x?,
            Err(e) => {
                // clean up after ourselves
                for i in 0..written_indexes {
                    unsafe {
                        ptr::drop_in_place(array[i].as_mut_ptr());
                    }
                }
                panic::resume_unwind(e)
            }
        }
        // technically [I::Item; N] is a dependently sized type, so transmute
        // is out.
        Some(unsafe { array_assume_init(array) })
    }
}
