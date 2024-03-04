use std::io;

#[cfg(target_family = "unix")]
#[link(name = "c")]
extern "C" {
    fn write(fd: u32, buf: *const u8, count: usize) -> i32;
}

pub fn syscall(message: String) -> io::Result<()> {
    let msg_ptr = message.as_ptr();
    let len = message.len();
    println!("len={}", len);
    let res = unsafe { 
        write(1, msg_ptr, len)        
    };
    println!("\n");
    if res == -1 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}
