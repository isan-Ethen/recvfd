use std::fs::File;
use std::io::{self, Read};
use std::os::unix::io::{FromRawFd, RawFd};
use std::thread;

fn from_syscall_error(error: syscall::Error) -> io::Error {
    io::Error::from_raw_os_error(error.errno as i32)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fd_path = format!("chan:{}", "/tmp/unix-domain-socket/test");

    println!("receive file descriptor");
    let chan_fd =
        syscall::open(fd_path, syscall::O_RDWR | syscall::O_CREAT).map_err(from_syscall_error)?;

    println!("call named dup");
    let receiver_fd = syscall::dup(chan_fd, b"recvfd").map_err(from_syscall_error)?;
    println!("raw fd: {}", receiver_fd);

    thread::sleep(std::time::Duration::from_secs(3));

    println!("as raw fd");
    let mut file = unsafe { File::from_raw_fd(receiver_fd as RawFd) };

    let mut contents = String::new();
    println!("read to string");
    file.read_to_string(&mut contents)?;

    println!("file contents:\n{}", contents);

    Ok(())
}
