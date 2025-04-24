// Copyright 2025 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

mod context;
pub mod error;

pub use context::*;
pub use error::{Error, Result};
use error::{ErrorKind, ResultExt as _};
use std::path::Path;

pub trait Loader {
    fn load(path: impl AsRef<Path>, replace: bool) -> Result<()>;
}

#[cfg(feature = "dotenvy")]
pub fn load(replace: bool, context: Option<&AzdContext>) -> Result<()> {
    load_with::<Dotenvy>(replace, context)
}

pub fn load_with<T: Loader>(replace: bool, context: Option<&AzdContext>) -> Result<()> {
    let default_context;
    let context = match context {
        Some(context) => context,
        None => {
            default_context = AzdContext::builder().build()?;
            &default_context
        }
    };

    let environment_file = context.environment_file();
    T::load(&environment_file, replace).with_context_fn(ErrorKind::Io, || {
        format!(
            "failed to load environment variables from '{}",
            environment_file.display(),
        )
    })
}

#[cfg(feature = "dotenvy")]
mod dotenvy {
    use crate::error::{ErrorKind, ResultExt as _};

    #[derive(Debug)]
    pub struct Dotenvy;

    impl super::Loader for Dotenvy {
        fn load(path: impl AsRef<std::path::Path>, replace: bool) -> crate::Result<()> {
            if replace {
                ::dotenvy::from_path_override(&path)
            } else {
                ::dotenvy::from_path(&path)
            }
            .with_kind(ErrorKind::Io)
        }
    }
}

#[cfg(feature = "dotenvy")]
pub use dotenvy::Dotenvy;
