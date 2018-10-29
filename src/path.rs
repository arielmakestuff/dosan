// src/path.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::env;
use std::io;
use std::path::{Component, Path, PathBuf};

// Third-party imports
#[cfg(test)]
use mocktopus::macros::*;

// Local imports

// ===========================================================================
// PathExt
// ===========================================================================

// Helper for PathExt::normalize
//
// This processes a parent directory component
fn normalize_parentdir_comp(
    path: &Path,
    ret: &mut PathBuf,
    last_comp: &Option<Component>,
) -> io::Result<()>
{
    if ret.as_os_str().is_empty()
    {
        let curdir = path.current_dir()?;
        match curdir.parent()
        {
            Some(p) => ret.push(p),
            None =>
            {}
        }
    }
    else
    {
        match last_comp
        {
            None | Some(Component::Prefix(_)) | Some(Component::RootDir) =>
            {}

            Some(_) =>
            {
                ret.pop();
            }
        }
    }

    Ok(())
}

#[cfg_attr(test, mockable)]
pub trait PathExt: AsRef<Path>
{
    fn current_dir(&self) -> io::Result<PathBuf>
    {
        env::current_dir()
    }

    fn change_dir(&self) -> io::Result<()>
    {
        env::set_current_dir(self)
    }

    fn normalize(&self) -> io::Result<PathBuf>
    {
        let curpath = self.as_ref();
        let mut ret = PathBuf::new();
        let mut last_comp: Option<Component> = None;

        for comp in curpath.components()
        {
            match comp
            {
                Component::ParentDir =>
                {
                    normalize_parentdir_comp(curpath, &mut ret, &last_comp)?;
                }

                Component::CurDir =>
                {
                    if ret.as_os_str().is_empty()
                    {
                        ret = self.current_dir()?;
                    }
                }

                _ =>
                {
                    ret.push(comp);
                }
            }

            last_comp = Some(comp);
        }

        Ok(ret)
    }
}

impl PathExt for Path {}

impl PathExt for PathBuf {}

// ===========================================================================
//
// ===========================================================================
