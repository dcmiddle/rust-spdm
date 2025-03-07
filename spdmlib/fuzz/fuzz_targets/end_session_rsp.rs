// Copyright (c) 2023 Intel Corporation
//
// SPDX-License-Identifier: Apache-2.0

#![no_main]

use libfuzzer_sys::fuzz_target;

include!("../../../fuzz-target/responder/end_session_rsp/src/main.rs");

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    fuzz_handle_spdm_end_session(data);
});
