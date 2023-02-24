# `causal-hub`: A Package for Causal Data Science

[![build](https://github.com/AlessioZanga/causal-hub/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/AlessioZanga/causal-hub/actions/workflows/build.yml)
[![docs.rs](https://img.shields.io/docsrs/causal-hub)](https://docs.rs/causal-hub)
[![codecov](https://codecov.io/gh/AlessioZanga/causal-hub/branch/main/graph/badge.svg?token=n1VZoqizYC)](https://codecov.io/gh/AlessioZanga/causal-hub)

A package for Causal Data Science.

## Table of Contents

- [Overview](#overview)
- [Documentation](#documentation)
- [Changelog](#changelog)
- [Contributing](#contributing)
- [Citation](#citation)
- [License](#license)
- [Versioning](#versioning)

## Overview

Causal _inference_ is the process of identifying and estimating the causal effect of a given _treatment_ for a chosen _outcome_. To formally describe the relationship between a cause and its effect, a causal model must be constructed from the available data and the experts' prior knowledge in a process called causal _discovery_. This library is intended to collect, organize and exploit state of the art methodology to enable _causal data science_.

To use this software, run the following `cargo` command in your project directory:

    cargo add causal-hub

Or add the following lines to your `Cargo.toml`:

```toml
[dependencies]
causal-hub = "^0.1"
```

## Documentation

The official documentation is available [here](https://docs.rs/causal-hub).

## Changelog

All notable changes to this project will be documented in the [CHANGELOG](./CHANGELOG.md).

## Contributing

To contribute to this software refer to [CONTRIBUTING](./.github/CONTRIBUTING.md).

## Citation

To cite this software refer to [CITATION](./CITATION.cff) or click on `Cite this repository` in the GitHub repository. [Read more](https://citation-file-format.github.io).

## License

This software is distributed under the terms of both the Apache License (Version 2.0) and the MIT license.

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.

## Versioning

This software follows the [SemVer](https://semver.org/spec/v2.0.0.html) specification.
