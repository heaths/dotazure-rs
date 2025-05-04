// Copyright 2025 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

#![doc = include_str!("../README.md")]

mod context;
pub mod error;

pub use context::*;
pub use error::{Error, Result};
use error::{ErrorKind, ResultExt as _};

/// Load environment variables for an Azure Developer CLI project.
///
/// Locates the `.env` file from the default environment name if an Azure Developer CLI project was already provisioned.
/// Environment variables that were already set will not be replaced.
///
/// Call [`loader()`] to customize the behavior.
pub fn load() -> Result<()> {
    loader().load()
}

/// Get a builder to customize discovery and loading of environment variables.
pub fn loader() -> Loader {
    Loader::default()
}

/// A builder interface to customize discovery and loading of environment variables.
#[derive(Debug, Default)]
pub struct Loader {
    context: Option<AzdContext>,
    replace: bool,
}

impl Loader {
    /// Set the [`AzdContext`] to use for discovery.
    pub fn context(self, context: AzdContext) -> Self {
        Self {
            context: Some(context),
            ..self
        }
    }

    /// Set whether to replace environment variables that were already set.
    pub fn replace(self, replace: bool) -> Self {
        Self { replace, ..self }
    }

    /// Finds and loads the appropriate `.env` file.
    pub fn load(self) -> Result<()> {
        let context = match self.context {
            Some(context) => context,
            None => AzdContext::builder().build()?,
        };
        let path = context.environment_file();
        if self.replace {
            dotenvy::from_filename_override(&path)
        } else {
            dotenvy::from_filename(&path)
        }
        .with_context_fn(ErrorKind::Io, || {
            format!("failed to load {}", path.display())
        })?;

        Ok(())
    }
}
