// use libc::{
//     bind, connect,
//     header::{
//         arpa_inet::inet_aton,
//         errno::{EAFNOSUPPORT, EDOM, EFAULT, EINVAL, ENOSYS, EOPNOTSUPP, EPROTONOSUPPORT},
//         netinet_in::{in_addr, in_port_t, sockaddr_in},
//         string::strnlen,
//         sys_socket::{constants::*, msghdr, sa_family_t, sockaddr, socklen_t},
//         sys_time::timeval,
//         sys_un::sockaddr_un,
//     },
//     recvmsg, sendmsg, socket,
// };
use std::ffi::CString;
use std::fs::File;
use std::io::{self, Read};
use std::mem;
use std::os::raw::c_char;
use std::os::unix::io::{FromRawFd, RawFd};
use std::thread;

fn from_syscall_error(error: syscall::Error) -> io::Error {
    io::Error::from_raw_os_error(error.errno as i32)
}

// fn str2c_char_array(s: String) -> Result<[c_char; 108]> {
//     if s.len() > 108 {
//         eprintln!("path is longer than 108");
//         return Err(());
//     }
//     match CString::new(s) {
//         Ok(c_string) => {
//             let bytes = c_string.as_bytes_with_nul();
//
//             let mut array = [0 as c_char; 108];
//             for (i, &byte) in bytes.iter().enumerate().take(108) {
//                 array[i] = byte as c_char;
//             }
//             Ok(array)
//         }
//         Err(_) => Err(()),
//     }
// }
//
// unsafe fn listen_gate(path: String) -> Result<usize> {
//     let gate = socket(AF_UNIX, SOCK_DGRAM, 0);
//     if gate < 0 {
//         return Err(());
//     }
//
//     let sun_path: [c_char; 108] = str2c_char_array(path)?;
//
//     let gate_addr: sockaddr_un = sockaddr_un {
//         sun_family: syscall::AF_UNIX,
//         sun_path,
//     };
//
//     if bind(
//         gate,
//         &gate_addr as *const sockaddr,
//         mem::size_of_val::<socklen_t>(gate_addr),
//     ) < 0
//     {
//         eprintln!("bind gate error");
//         return Err(());
//     }
//
//     Ok(gate)
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fd_path = format!("chan:{}", "/tmp/unix-domain-socket/test");

    println!("receive file descriptor");
    let chan_fd =
        syscall::open(fd_path, syscall::O_RDWR | syscall::O_CREAT).map_err(from_syscall_error)?;

    // let gate = unsafe { listen_gate(&fd_path)? };

    println!("call named dup");
    let receiver_fd = syscall::dup(chan_fd, b"listen").map_err(from_syscall_error)?;
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
