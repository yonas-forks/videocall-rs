/*
 * Copyright 2025 Security Union LLC
 *
 * Licensed under either of
 *
 * * Apache License, Version 2.0
 *   (http://www.apache.org/licenses/LICENSE-2.0)
 * * MIT license
 *   (http://opensource.org/licenses/MIT)
 *
 * at your option.
 *
 * Unless you explicitly state otherwise, any contribution intentionally
 * submitted for inclusion in the work by you, as defined in the Apache-2.0
 * license, shall be dual licensed as above, without any additional terms or
 * conditions.
 */

//! A high-fidelity, cross-platform video decoder jitter buffer implementation in Rust.

pub mod decoder;
#[cfg(not(target_arch = "wasm32"))]
pub mod encoder;
pub mod frame;
pub mod jitter_buffer;
pub mod jitter_estimator;
