// Copyright 2025 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

mod context;
pub mod error;

pub use context::*;
pub use error::{Error, Result};
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
    T::load(environment_file, replace)
}

#[cfg(feature = "dotenvy")]
mod dotenvy {
    use crate::error::{Error, ErrorKind};

    #[derive(Debug)]
    pub struct Dotenvy;

    impl super::Loader for Dotenvy {
        fn load(path: impl AsRef<std::path::Path>, replace: bool) -> crate::Result<()> {
            if replace {
                ::dotenvy::from_path_override(path)?;
            } else {
                ::dotenvy::from_path(path)?;
            }
            Ok(())
        }
    }

    impl From<::dotenvy::Error> for Error {
        fn from(err: ::dotenvy::Error) -> Self {
            Error::with_error(ErrorKind::Io, err, "failed to load environment variables")
        }
    }
}

#[cfg(feature = "dotenvy")]
pub use dotenvy::Dotenvy;
