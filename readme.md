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