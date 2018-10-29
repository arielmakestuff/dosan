// src/test/path.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports

// Third-party imports

// Local imports

// ===========================================================================
// Tests
// ===========================================================================

// --------------------
// change_dir
// --------------------

// Properties:
// 1. Called on a valid path that is the current directory, does nothing and
//    returns ok
//
// 2. Called on a valid, existing directory path, changes current directory to
//    that path and returns ok
//
// 3. Called on a valid path that is a file, does nothing and returns err
//
// 4. Called on an invalid path, does nothing and returns err
//
// 5. Called on a valid path that doesn't exist, does nothing and returns err
//

// --------------------
// current_dir
// --------------------
// 1. Called when current directory exists, returns PathBuf of current dir
//
// 2. Called when current directory doesn't exist, returns err

// --------------------
// Current directory exists
// --------------------
// 1. Called when current directory exists, returns PathBuf of current dir

use crate::path::PathExt;
use std::io;
use std::path::{Path, PathBuf};

use mocktopus::mocking::*;

// #[mockable]
// fn answer() -> u8
// {
//     42
// }

#[test]
fn valid_dir_noop_ok()
{
    impl PathExt for Path {}

    let expected_path = "/some/path";
    let expected = PathBuf::from(expected_path);
    Path::current_dir.mock_safe(move || {
        MockResult::Return(Ok(PathBuf::from(&expected_path)))
    });

    let result = Path::current_dir();

    let assert_result = match result
    {
        Ok(p) => p == expected,
        _ => false,
    };
    assert!(assert_result);
}

// ===========================================================================
//
// ===========================================================================
