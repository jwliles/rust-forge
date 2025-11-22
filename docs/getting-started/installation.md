---
date_created: 2025-10-03T02-46-38
date_updated: 2025-10-03T02-46-38
timestamp: 1759459598162
title: installation
id: fa974d16-ff55-4ba6-a74c-61eaf176d451
hash: 822e65d4c7158243b40dac1f160110e49fb6c97d3b3c2d0d66134a21a00be1b1
---
# Installation

## From crates.io

The recommended way to install Forge is through cargo:

```bash
cargo install forge-rs
```

## From Source

You can also build and install from source:

```bash
# Clone the repository
git clone https://github.com/jwliles/rust-forge.git
cd rust-forge

# Build and install
cargo install --path .
```

## Verifying Installation

To verify that Forge is installed correctly, run:

```bash
forge --version
```

You should see the version number of the installed Forge.