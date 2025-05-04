// Copyright 2025 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use crate::error::{Error, ErrorKind, Result};
use serde::Deserialize;
use std::{
    env,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

const CONFIG_FILE_NAME: &str = "config.json";
const ENVIRONMENT_DIR_NAME: &str = ".azure";
const ENVIRONMENT_FILE_NAME: &str = ".env";
const PROJECT_FILE_NAME: &str = "azure.yaml";

/// Project information for the Azure Development CLI.
#[derive(Clone, Debug)]
pub struct AzdContext {
    project_dir: PathBuf,
    environment_name: String,
}

impl AzdContext {
    /// Gets an [`AzdContextBuilder`] to construct an `AzdContext`.
    pub fn builder() -> AzdContextBuilder {
        AzdContextBuilder::default()
    }

    /// Gets the directory containing the `azure.yaml` project file.
    pub fn project_dir(&self) -> &Path {
        self.project_dir.as_path()
    }

    /// Gets the path to the `azure.yaml` project file.
    pub fn project_path(&self) -> PathBuf {
        self.project_dir.join(PROJECT_FILE_NAME)
    }

    /// Gets the path to the `.azure` directory.
    pub fn environment_dir(&self) -> PathBuf {
        self.project_dir.join(ENVIRONMENT_DIR_NAME)
    }

    /// Gets the name of the environment.
    pub fn environment_name(&self) -> &str {
        &self.environment_name
    }

    /// Gets the path to the environment directory under [`AzdContext::environment_dir()`].
    pub fn environment_root(&self) -> PathBuf {
        self.environment_dir().join(&self.environment_name)
    }

    /// Gets the path to the `.env` file under [`AzdContext::environment_root()`].
    pub fn environment_file(&self) -> PathBuf {
        self.environment_root().join(ENVIRONMENT_FILE_NAME)
    }
}

/// A builder to construct an [`AzdContext`].
#[derive(Debug, Default)]
pub struct AzdContextBuilder {
    current_dir: Option<PathBuf>,
    environment_name: Option<String>,
}

impl AzdContextBuilder {
    /// Sets the current directory.
    ///
    /// The default is [`std::env::current_dir()`].
    pub fn current_dir(self, path: impl Into<PathBuf>) -> Result<Self> {
        let current_dir: PathBuf = path.into();
        if !current_dir.exists() {
            return Err(Error::new(
                ErrorKind::Io,
                format!("{} does not exist", current_dir.display()),
            ));
        }

        Ok(Self {
            current_dir: Some(current_dir),
            ..self
        })
    }

    /// Sets the environment name.
    ///
    /// The default comes from the `AZURE_ENV_NAME` environment variable
    /// or from the `.azure/config.json` file.
    pub fn environment_name(self, name: impl Into<String>) -> Result<Self> {
        let environment_name: String = name.into();
        if environment_name.is_empty() {
            return Err(Error::new(ErrorKind::InvalidData, "name cannot be empty"));
        }

        Ok(Self {
            environment_name: Some(environment_name),
            ..self
        })
    }

    /// Constructs an [`AzdContext`].
    pub fn build(self) -> Result<AzdContext> {
        let current_dir = match self.current_dir {
            Some(path) => path,
            None => env::current_dir()?,
        };

        let mut project_dir = None;
        for dir in current_dir.ancestors() {
            let path = dir.join(PROJECT_FILE_NAME);
            if path.exists() {
                project_dir = Some(dir);
                break;
            }
        }

        let Some(project_dir) = project_dir.map(Into::<PathBuf>::into) else {
            return Err(Error::new(
                ErrorKind::Io,
                "no project exists; to create a new project, run `azd init`",
            ));
        };

        let environment_name = match self.environment_name {
            Some(name) => name,
            None => {
                let path = project_dir
                    .join(ENVIRONMENT_DIR_NAME)
                    .join(CONFIG_FILE_NAME);
                let reader = BufReader::new(File::open(&path)?);
                let config: Config = serde_json::from_reader(reader)?;

                config.default_environment.ok_or_else(|| {
                    Error::new(
                        ErrorKind::InvalidData,
                        format!("'{}' does not define `defaultEnvironment`", path.display()),
                    )
                })?
            }
        };

        Ok(AzdContext {
            project_dir,
            environment_name,
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Config {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    default_environment: Option<String>,
}
