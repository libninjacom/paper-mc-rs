<div id="top"></div>

<p align="center">
    <a href="https://github.com/libninjacom/paper-mc-rs/graphs/contributors">
        <img src="https://img.shields.io/github/contributors/libninjacom/paper-mc-rs.svg?style=flat-square" alt="GitHub Contributors" />
    </a>
    <a href="https://github.com/libninjacom/paper-mc-rs/stargazers">
        <img src="https://img.shields.io/github/stars/libninjacom/paper-mc-rs.svg?style=flat-square" alt="Stars" />
    </a>
    <a href="https://github.com/libninjacom/paper-mc-rs/actions">
        <img src="https://img.shields.io/github/workflow/status/libninjacom/paper-mc-rs/test?style=flat-square" alt="Build Status" />
    </a>
    
<a href="https://crates.io/crates/paper-mc">
    <img src="https://img.shields.io/crates/d/paper-mc?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/paper-mc">
    <img src="https://img.shields.io/crates/v/paper-mc?style=flat-square" alt="Crates.io" />
</a>

</p>

PaperMC client, generated from the OpenAPI spec.

# Usage

```rust
use paper_mc::PaperMcClient;
use paper_mc::model::*;
#[tokio::main]
async fn main() {
    let client = PaperMcClient::from_env();
    let response = client.projects().send().await.unwrap();
    println!("{:#?}", response);
}

```

This example loads configuration from environment variables, specifically:

* `PAPER_MC_BASE_URL`



# Installation

Add this to your Cargo.toml:

```toml
[dependencies]
paper-mc = "0.1.0"
```


# Documentation



* [Client Library Documentation](https://docs.rs/paper-mc)


You can see working examples of every API call in the `examples/` directory.

# Contributing

Contributions are welcome!

*Library created with [Libninja](https://www.libninja.com).*