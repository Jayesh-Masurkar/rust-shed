/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This software may be used and distributed according to the terms of the
 * GNU General Public License found in the LICENSE file in the root
 * directory of this source tree.
 */

use fbinit::FacebookInit;

#[fbinit::test]
fn test_without_proof() {}

#[fbinit::test]
fn test_with_proof(fb: FacebookInit) {
    println!("Got fb: {:?}", fb);
}

/// This can work only on fbcode builds as only then the proof can be asserted
#[cfg(fbcode_build)]
#[fbinit::test]
fn test_expect_init() {
    fbinit::expect_init();
}

/// On non-fbcode builds asserting the proof will always panic, even in fbinit::test
#[cfg(not(fbcode_build))]
#[fbinit::test]
#[should_panic]
fn test_expect_init() {
    fbinit::expect_init();
}

#[test]
#[should_panic]
fn test_expect_init_panics() {
    fbinit::expect_init();
}

/// This can work only on fbcode builds as only then the proof can be asserted
#[cfg(fbcode_build)]
#[fbinit::test]
fn test_main_expect_init() {
    #[fbinit::main]
    fn main() {
        fbinit::expect_init();
    }

    main();
}

/// On non-fbcode builds asserting the proof will always panic, even in fbinit::test
#[cfg(not(fbcode_build))]
#[fbinit::test]
#[should_panic]
fn test_main_expect_init() {
    #[fbinit::main]
    fn main() {
        fbinit::expect_init();
    }

    main();
}

#[fbinit::test]
async fn test_async_without_proof() {
    async fn helper() {}

    helper().await;
}

#[fbinit::test]
async fn test_async_with_proof(fb: FacebookInit) {
    async fn helper(_fb: FacebookInit) {}

    helper(fb).await;
}

#[test]
fn test_main_without_proof() {
    #[fbinit::main]
    fn main() {}

    main();
}

#[test]
fn test_main_with_proof() {
    #[fbinit::main]
    fn main(fb: FacebookInit) {
        println!("Got fb: {:?}", fb);
    }

    main();
}

mod submodule {
    #[fbinit::main]
    fn main() {}

    #[test]
    #[should_panic(expected = "fbinit must be performed in the crate root on the main function")]
    fn test_in_submodule() {
        main();
    }
}
