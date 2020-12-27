// Exercises and examples from Programming Rust
// Excessive commenting for understanding

// ints can use u/i for signed and unsigned and 32/64 for the word size
// ! char after assert! or println! invokes a macro rather than a function call
// assert!() checks the condition passed to it and if it is false, terminates the
// program.


// a function which takes two parameters: two mutable unsigned
// 64 bit values and returns an unsigned 64-bit value
fn gcd( mut n: u64, mut m, u64) -> u64 {
    // assert - make sure these values are not present
    assert!(n != 0 && m != 0);

    // does not need () around condition
    while m!= 0{
        if m < n{
            // Rust can infer the type of t as u64 from m & n
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    // if a function ends with a statement not followed by a semicolon
    // it is treated as the functions return statement
    n
}

// an attribute: marks the function as a test function which is to be skipped in normal compilations
#[test]
fn test_gcd(){
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
    3 * 7 * 11 * 13 * 19), 3 * 11);
}

