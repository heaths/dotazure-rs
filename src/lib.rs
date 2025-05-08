// Copyright 2025 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

#![doc = include_str!("../README.md")]

mod context;
pub mod error;

pub use context::*;
use error::ErrorKind;
pub use error::{Error, Result};

/// Load environment variables for an Azure Developer CLI project.
///
/// Locates the `.env` file from the default environment name if an Azure Developer CLI project was already provisioned.
/// Returns `true` if the `.env` file was found and loaded successfully or `false` if no `.env` file was found.
/// Any errors navigating directories, or reading or parsing files will return an error.
///
/// Environment variables that were already set will not be replaced.
///
/// Call [`loader()`] to customize the behavior.
pub fn load() -> Result<bool> {
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
    ///
    /// Returns `true` if the `.env` file was found and loaded successfully or `false` if no `.env` file was found.
    /// Any errors navigating directories, or reading or parsing files will return an error.
    pub fn load(self) -> Result<bool> {
        let context = if let Some(context) = self.context {
            context
        } else {
            match AzdContext::builder().build() {
                Ok(context) => context,
                Err(err) if *err.kind() == ErrorKind::NotFound => return Ok(false),
                Err(err) => return Err(err),
            }
        };

        let path = context.environment_file();
        match if self.replace {
            dotenvy::from_filename_override(&path)
        } else {
            dotenvy::from_filename(&path)
        }
        .map_err(Into::<Error>::into)
        {
            Ok(_) => Ok(true),
            Err(err) if *err.kind() == ErrorKind::NotFound => Ok(false),
            Err(err) => Err(err),
        }
    }
}
