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
// is_current_dir
// --------------------
// 1. Called on a valid path that is the current dir, returns true
// 2. Called on a valid path that is a file, returns false
// 3. Called on a valid path that is a dir but not the current dir, returns
//    false
// 4. Called on an invalid path, returns false

// --------------------
// is_current dir
// --------------------
// 1. Called on a valid path that is the current dir, returns true

use crate::path::PathExt;
use std::io;
use std::mem;
use std::path::{Path, PathBuf};

use mocktopus::mocking::*;

#[test]
fn dir_is_current()
{
    // --------------------
    // GIVEN
    // --------------------
    // A Path instance that is the current directory and

    let curdir = "/current/dir";
    let mut dirs_returned =
        vec![PathBuf::from(&curdir), PathBuf::from("some/other/dir")];
    let mut dirs_iter = dirs_returned.iter_mut();

    let mut init_path = PathBuf::from("init");
    let current_dir = &mut init_path;

    let mut set_curdir = |curdir: &mut PathBuf| {
        mem::swap(current_dir, curdir);
    };

    unsafe {
        Path::current_dir.mock_raw(move |_| {
            let next = dirs_iter.next().unwrap();
            let ret = next.clone();
            set_curdir(next);
            MockResult::Return(Ok(ret))
        });
    }

    // --------------------
    // WHEN
    // --------------------
    // PathExt::change_dir() is called on the Path instance

    let current = Path::new(&curdir);
    let result = current.is_current_dir();

    // --------------------
    // THEN
    // --------------------
    // true is returned

    assert_eq!(current_dir.as_path(), PathBuf::from(&curdir).as_path());
    assert!(result);
}

// ===========================================================================
//
// ===========================================================================
