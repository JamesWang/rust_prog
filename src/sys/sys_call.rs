use std::arch::asm;

#[inline(never)]
pub fn syscall(message: String) {
    let msg_ptr = message.as_ptr();
    let len: usize = message.len();
    unsafe {
        asm!(
            "mov rax, 1",
            "mov rdi, 1",
            "syscall",
            in("rsi") msg_ptr,
            in("rdx") len,
            out("rax") _,
            out("rdi") _,
            lateout("rsi") _,
            lateout("rdx") _
        );
    }
}

pub fn sys_main() {
    let message = "Hello world from raw syscall!\n";
    let message = String::from(message);
    syscall(message)
}