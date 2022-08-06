# Rust Snippet - Mutex with concurrent tests

## Purpose
To demonstrate adding a mutex within a shared resource.

## Standard Run
**cargo test**

    running 2 tests
    test test_0_setvalue ... ok
    test test_1_readvalue ... ok

    test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.00s



## Debug run
**cargo test -- --nocapture**

    running 2 tests
    test_0_setvalue RUN
    test_0_setvalue END
    test_1_readvalue RUN
    test test_0_setvalue ... ok
    test_1_readvalue END
    test test_1_readvalue ... ok

    test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.00s



