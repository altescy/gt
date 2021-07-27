GT
==

[![Actions Status](https://github.com/altescy/gt/workflows/CI/badge.svg)](https://github.com/altescy/gt/actions?query=workflow%3ACI)
[![License](https://img.shields.io/github/license/altescy/gt)](https://github.com/altescy/gt/blob/master/LICENSE)
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/altescy/gt)](https://github.com/altescy/gt/)

Generate template of gitignore / license via [GitHub REST API](https://docs.github.com/en/rest).

## Installation

```
cargo install --git https://github.com/altescy/gt --tag v1.0.0
```

## Usage

#### Generate gitignore or license

```
❯ gt generate rust,python > .gitignore
❯ gt generate mit > LICENSE
```

#### Show the list of available names

```
❯ gt -t license list
agpl-3.0
apache-2.0
bsd-2-clause
...
```
