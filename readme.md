An implementation of Caesar Cypher using the TDD framework in Rust.

Usage example :

To encrypt a message :
    $cargo run "baptiste" 14
        -> "podhwghs"

To decrypt a message :
    $cargo run "podhwghs" -14
        -> "baptiste"

To see if you pass the units tests :
    $cargo test
