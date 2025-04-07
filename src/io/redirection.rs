use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::io::{AsRawFd, RawFd}; // For Unix-specific file descriptor manipulation

// Redirect output to a file (>)
pub fn redirect_output_to_file(filename: &str) -> io::Result<RawFd> {
    let file = File::create(filename)?;
    let fd = file.as_raw_fd();

    // Duplicate the file descriptor for stdout
    unsafe {
        libc::dup2(fd, libc::STDOUT_FILENO);
    }

    Ok(fd)
}

// Append output to a file (>>)
pub fn redirect_output_to_file_append(filename: &str) -> io::Result<RawFd> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)?;
    let fd = file.as_raw_fd();

    // Duplicate the file descriptor for stdout
    unsafe {
        libc::dup2(fd, libc::STDOUT_FILENO);
    }

    Ok(fd)
}

// Redirect input from a file (<)
pub fn redirect_input_from_file(filename: &str) -> io::Result<RawFd> {
    let file = File::open(filename)?;
    let fd = file.as_raw_fd();

    // Duplicate the file descriptor for stdin
    unsafe {
        libc::dup2(fd, libc::STDIN_FILENO);
    }

    Ok(fd)
}

// Restore standard file descriptors
pub fn restore_io(original_stdin: Option<RawFd>, original_stdout: Option<RawFd>) {
    if let Some(fd) = original_stdin {
        unsafe {
            libc::dup2(fd, libc::STDIN_FILENO);
            libc::close(fd);
        }
    }

    if let Some(fd) = original_stdout {
        unsafe {
            libc::dup2(fd, libc::STDOUT_FILENO);
            libc::close(fd);
        }
    }
}
