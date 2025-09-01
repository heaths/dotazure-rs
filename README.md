# Dotazure

[![releases](https://img.shields.io/github/v/release/heaths/dotazure-rs.svg?logo=github)](https://github.com/heaths/dotazure-rs/releases/latest)
[![docs](https://img.shields.io/docsrs/dotazure?logo=rust)](https://docs.rs/dotazure)
[![ci](https://github.com/heaths/dotazure-rs/actions/workflows/ci.yml/badge.svg?event=push)](https://github.com/heaths/dotazure-rs/actions/workflows/ci.yml)

Locate and load environment variables defined when provisioning an [Azure Developer CLI] project.

## Getting Started

If you do not already have an [Azure Developer CLI](azd) project, you can create one:

```sh
azd init
```

Add [`dotazure`](https://crates.io/crates/dotazure) to your project:

```sh
cargo add dotazure
```

After you define some resources e.g., an [Azure Key Vault](https://github.com/heaths/dotazure-rs/blob/main/infra/resources.bicep),
you can provision those resources which will create a `.env` file with any `output` parameters:

```sh
azd up
```

## Example

After `azd up` provisions resources and creates a `.env` file, you can call `load()` to load those environment variables
from the default environment e.g.,

```rust no_run
fn main() {
    dotazure::load().unwrap();

    // Assumes bicep contains e.g.
    //
    // output AZURE_KEYVAULT_URL string = kv.properties.vaultUri
    println!(
        "AZURE_KEYVAULT_URL={}",
        std::env::var("AZURE_KEYVAULT_URL").unwrap(),
    );
}
```

If you want to customize behavior, you can call `dotazure::loader()` to get a builder-like object.

## License

Licensed under the [MIT](https://github.com/heaths/dotazure-rs/blob/refactor/LICENSE.txt) license.

[Azure Developer CLI]: https://aka.ms/azd
