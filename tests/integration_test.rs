// Integration test for SigmaOS-mint-executor
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use mint_executor as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
