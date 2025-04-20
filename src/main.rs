use libc::{bind, socket, write};
use std::ffi::CString;
use std::fs::File;
use std::io::{self, Read};
use std::mem;
use std::os::unix::io::{FromRawFd, RawFd};
use std::thread;

type Result<T> = std::result::Result<T, io::Error>;

fn from_syscall_error(error: syscall::Error) -> io::Error {
    io::Error::from_raw_os_error(error.errno as i32)
}

fn print_fds() -> Result<()> {
    let files_fd = syscall::open(
        "/scheme/thisproc/current/filetable",
        syscall::O_RDONLY as usize,
    )
    .map_err(from_syscall_error)? as RawFd;
    let mut file = unsafe { File::from_raw_fd(files_fd) };
    let mut contents = String::new();
    println!("read to string");
    file.read_to_string(&mut contents)?;

    Ok(())
}

fn main() -> Result<()> {
    print_fds()?;
    println!("file contents:\n{}", contents);

    let fd_path = "/tmp/uds/test";
    let scheme_path = format!("/scheme/chan{}", fd_path);
    println!("scheme path: {}", scheme_path);

    println!("listen gate");
    let scheme_fd = syscall::open(scheme_path, syscall::O_CREAT | syscall::O_RDWR)
        .map_err(from_syscall_error)?;
    println!("accept socket");
    let gate = syscall::dup(scheme_fd, b"listen").map_err(from_syscall_error)?;
    print_fds()?;
    println!("call named dup");
    let fd = syscall::dup(gate, b"/scheme/file/home/user/test").map_err(from_syscall_error)?;
    println!("raw fd: {}", fd);

    let message = "hello from receiver";
    let res = unsafe {
        write(
            fd,
            message.as_ptr() as *const std::os::raw::c_void,
            message.len(),
        )
    };

    Ok(())
}
