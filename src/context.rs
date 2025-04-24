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

#[derive(Debug)]
pub struct AzdContext {
    project_dir: PathBuf,
    environment_name: String,
}

impl AzdContext {
    pub fn builder() -> AzdContextBuilder {
        AzdContextBuilder::default()
    }

    pub fn project_dir(&self) -> &Path {
        self.project_dir.as_path()
    }

    pub fn project_path(&self) -> PathBuf {
        self.project_dir.join(PROJECT_FILE_NAME)
    }

    pub fn environment_dir(&self) -> PathBuf {
        self.project_dir.join(ENVIRONMENT_DIR_NAME)
    }

    pub fn environment_root(&self) -> PathBuf {
        self.environment_dir().join(&self.environment_name)
    }

    pub fn environment_file(&self) -> PathBuf {
        self.environment_root().join(ENVIRONMENT_FILE_NAME)
    }
}

#[derive(Debug, Default)]
pub struct AzdContextBuilder {
    current_dir: Option<PathBuf>,
    environment_name: Option<String>,
}

impl AzdContextBuilder {
    pub fn current_dir(self, path: impl Into<PathBuf>) -> Self {
        Self {
            current_dir: Some(path.into()),
            ..self
        }
    }

    pub fn environment_name(self, name: impl Into<String>) -> Self {
        Self {
            environment_name: Some(name.into()),
            ..self
        }
    }

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
