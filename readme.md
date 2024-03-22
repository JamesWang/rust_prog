### Guide
- Use functions from lib.rs
        
        use <module_name>::func_name
        i.e:
        use rust_prog::gcd
 
- Write tests
        
        #[cfg(test)]
        mod tests {
            use super::*;
        }

- Results from u64::from_str
        
        from_str() returns Result value with:
                    Result
                    /    \
                 Ok(v)   Err(e)

        Ok(v) indicateing succeeded and v is the value
        Err(e) indicating failure and e is an error value
- Ownership
  &numbers to tell Rush that ownership of the vector(numbers) should remain
  with numbers; just borrowing its element for the loop.
  - &   borrows a reference to the vector's elements
  - \*   *m, dereference m, yielding the value it refers to

  numbers own the vector, when numbers goes out of scope, Rust will automatically free it

- When get error:
  
       error: cannot find derive macro `Deserialize` in this scope

  This means something missing in Cargo.toml for serde, should enable derive feature on serde:

       i. e
       serde = { version = "1.0", features = ["derive"] }

- Run test using cargo with output:

      cargo test -- --nocapture
      or
      cargo test -- --show-output

- Monomorphization 
      
      Is the process of turning generic code into specific code by filling in the concrete
      types that are used when compiled.
      Rust compiles generic code into code that specifies the type in each instance,
      no runtime cost for using generics.


## About Fn/FnMut/FnOnce and Variance/Covariance/Contravariance

- Fn is sub-trait of FnMut, then FnMut is sub-trait of FnOnce
- Closure will be compiled to implement one or more of them during compiling
- An api high order function which accepts FnOnce, you can pass FnMut or Fn to it, 
  this follows parameters are **covariance**
- All closures implement **FnOnce**

## About CPU

- MMU - Memory Management Unit
        The MMU's job is to translate the virtual address we use in our program to physical address
- When the OS starts a process, it sets up a page table for our process and make sure a special register on the CPU points to this page table
- OS provided a pointer to a function that handles page fault, the CPU jumps to that function when we try to dereference 99999999999999 and thereby hands over control to the operating system.
  
## About Thread, GreeThread, and Fiber

- **Cooperative**: yield voluntarily either by explicitly yielding or by call a function that suspends the task when it cannot progress futher
  - Examples are tasks generated by ***async/await*** in Rust and Javascript
- **Non-cooperative**: The scheduler must be able to **pre-empt** a running task
  - Examples are OS Thread and Goroutine
- Stackful: Eash task has its own call stack, stack occupy a fixed size of memory, and (usually) cannot grow or thrink.
  - Fiber
  - Green thread (Virtual Thread)
  **Can suspend execution at any point in the program as the whole stack is preserved.**
**Coroutine has stackful corountine and stackless corountine**

- Stackless: There is not a separate stack for each task, thye are all run sharing the same call stack
  - Coroutine(Rust Futures generated by async/await)
- Every time you yield to the OS, you're put in the same queue as all other threads and processes on the system.
- No guarantee that the thread will resume execution on the same CPU core as it left off.
- Green threads and fibers use the same mechanisms as an OS, setting up a stack for each task. saving the CPU's state, and jumping from one task to another by doing a context switch. The state of execution is stored in each stack, so in such a solution, there would be no need for async/await, Future or Pin.
- A runtime using fibers/green-threads for concurrent tasks can have a high degree of flexiblity. Tasks can be pre-empted and context switched ***at any time and at any point in their execution***, this gives the runtime scheduler almost the same capabilities as the OS Scheduler, which is one of the biggest advantages of system using fibers/green-threads
- As fibers and green threads are similar to OS threads, they do have some of the same drawbacks as
well. Each task is set up with a stack of a fixed size, so you still have to reserve more space than you
actually use. However, these stacks can be growable, meaning that once the stack is full, the runtime
can grow the stack.
