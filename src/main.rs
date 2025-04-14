use std::fs::File;
use std::io::{self, Read};
use std::os::unix::io::{FromRawFd, RawFd};

fn from_syscall_error(error: syscall::Error) -> io::Error {
    io::Error::from_raw_os_error(error.errno as i32)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fd_path = format!("chan:{}", "/tmp/unix-domain-socket/test");

    println!("receivd file descriptor");
    let received_fd = syscall::open(fd_path, syscall::O_RDWR).map_err(from_syscall_error)?;

    println!("raw fd: {}", received_fd);

    println!("as raw fd");
    let mut file = unsafe { File::from_raw_fd(received_fd as RawFd) };

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("File contents:\n{}", contents);

    Ok(())
}
