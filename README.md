# Data Race in Rust: Multiple Mutable References

This repository demonstrates a potential data race in Rust. The example focuses on creating multiple mutable references to the same variable without proper synchronization, leading to undefined behavior. 

**Bug:**
The `bug.rs` file contains code that attempts to create two mutable references (`y` and `z`) to the variable `x` simultaneously. This violates Rust's borrowing rules and can cause unpredictable results, including data races, crashes, or incorrect outputs.

**Solution:**
The `bugSolution.rs` file offers a corrected version of the code.  It avoids the data race by ensuring that only one mutable reference to `x` exists at any given time. 
