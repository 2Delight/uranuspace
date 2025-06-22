use uranuspace::bindings::syscalls::getcwd;
use uranuspace::bindings::types::{c_char, size_t};

const SIZE: size_t = 1000;

fn main() {
    let mut name: [c_char; SIZE] = [0; SIZE];

    unsafe {
        getcwd(name.as_mut_ptr(), SIZE);
    }

    if name == [0; SIZE] {
        println!("failed to print working directory");
        return;
    }

    match std::ffi::CStr::from_bytes_until_nul(
        name.iter().map(|&b| b as u8).collect::<Vec<_>>().as_slice(),
    ) {
        Ok(path) => match path.to_str() {
            Ok(path) => println!("{}", path),
            Err(err) => println!("failed to print working directory: {}", err),
        },
        Err(err) => println!("failed to print working directory: {}", err),
    }
}
