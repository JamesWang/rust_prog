use std::arch::asm;

const DEFAULT_STACK_SIZE: usize = 1024 * 1024; // 1 MB
const MAX_THREADS: usize = 4;
static mut RUNTIME: usize = 0;

pub struct Runtime {
    threads: Vec<Thread>,
    current: usize,
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum State {
    Ready,
    Running,
    Available,
}

struct Thread {
    state: State,
    stack: Vec<u8>,
    context: ThreadContext,
}

impl Thread {
    fn new() -> Self {
        Thread {
            state: State::Available,
            stack: vec![0_u8; DEFAULT_STACK_SIZE],
            context: ThreadContext::default(),
        }
    }
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
struct ThreadContext {
    rsp: u64,
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbx: u64,
    rbp: u64,
}

impl Runtime {
    pub fn new() -> Self {
        let base_thread: Thread = Thread {
            state: State::Running,
            stack: vec![0_u8; DEFAULT_STACK_SIZE],
            context: ThreadContext::default(),
        };

        let mut threads = vec![base_thread];
        let mut available_threads: Vec<Thread> = (1..MAX_THREADS).map(|_| Thread::new()).collect();
        threads.append(&mut available_threads);
        Runtime {
            threads,
            current: 0,
        }
    }
    pub fn init(&self) {
        unsafe {
            let r_ptr: *const Runtime = self;
            RUNTIME = r_ptr as usize;
        }
    }

    pub fn run(&mut self) {
        while self.t_yield() {}
        std::process::exit(0);
    }

    #[inline(never)]
    pub fn t_yield(&mut self) -> bool {
        let mut current = self.current;
        while self.threads[current].state != State::Ready {
            current += 1;
            if current == self.threads.len() {
                current = 0;
            }
            if current == self.current {
                return false; // no runnable threads
            }
        }
        if self.threads[self.current].state == State::Available {
            self.threads[self.current].state = State::Ready;
        }
        self.threads[current].state = State::Running;
        let prev = self.current;
        self.current = current;
        unsafe {
            let old: *mut ThreadContext = &mut self.threads[prev].context;
            let new: *const ThreadContext = &self.threads[current].context;
            asm!(
                "call switch",
                in("rdi") old,
                in("rsi") new,
                clobber_abi("C"),
            );
        }
        self.threads.len() > 0
    }

    fn t_return(&mut self) {
        let current = self.current;
        if current != 0 {
            self.threads[current].state = State::Available;
            self.t_yield();
        }
    }

    pub fn spawn(&mut self, f: fn()) {
        let available = self
            .threads
            .iter_mut()
            .find(|t| t.state == State::Available)
            .expect("No available threads");
        let size = available.stack.len();
        unsafe {
            let stack_top = available.stack.as_mut_ptr().offset(size as isize);
            let stack_top_aligned = (stack_top as usize & !15) as *mut u8;

            std::ptr::write(stack_top_aligned.offset(-16) as *mut u64, guard as u64);
            std::ptr::write(stack_top_aligned.offset(-24) as *mut u64, skip as u64);
            std::ptr::write(stack_top_aligned.offset(-32) as *mut u64, f as u64);
            available.context.rsp = (stack_top_aligned.offset(-32)) as u64;
        }
        available.state = State::Ready;
    }

    pub fn yield_thread() {
        unsafe {
            let r_ptr = RUNTIME as *mut Runtime;
            (*r_ptr).t_yield();
        };
    }
}

fn guard() {
    unsafe {
        let r_ptr = RUNTIME as *mut Runtime;
        (*r_ptr).t_return();
    };
}

pub fn yield_thread() {
    unsafe {
        let rt_ptr = RUNTIME as *mut Runtime;
        (*rt_ptr).t_yield();
    };
}

#[naked]
unsafe extern "C" fn skip() {
    std::arch::naked_asm!("ret");
}

#[naked]
#[no_mangle]
unsafe extern "C" fn switch() {
    std::arch::naked_asm!(
        // Save rsp to old context
        "mov [rdi + 0x00], rsp", // offset 0
        "mov [rdi + 0x08], r15", // offset 8
        "mov [rdi + 0x10], r14", // offset 16
        "mov [rdi + 0x18], r13", // offset 24
        "mov [rdi + 0x20], r12", // offset 32
        "mov [rdi + 0x28], rbx", // offset 40
        "mov [rdi + 0x30], rbp", // offset 48
        // Load rsp from new context
        "mov rsp, [rsi + 0x00]", // offset 0
        "mov r15, [rsi + 0x08]", // offset 8
        "mov r14, [rsi + 0x10]", // offset 16
        "mov r13, [rsi + 0x18]", // offset 24
        "mov r12, [rsi + 0x20]", // offset 32
        "mov rbx, [rsi + 0x28]", // offset 40
        "mov rbp, [rsi + 0x30]", // offset 48
        "ret",
    );
}
