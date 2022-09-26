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