cargo test -- --test-threads=1
cargo test -- --show-output # show std print in test
cargo test one_hundred # run single test
cargo test add # Filtering to run multiple tests
cargo test -- --ignored # do not ignore test marked as #[ignore]
