# swap.io clmm Rust SDK

This repository provides a Rust Software Development Kit (SDK) to interact with the Swap.io CLMM smart contracts and integrate with the [jup.ag swap router program](https://station.jup.ag/docs/dex-integration). It is designed to streamline the process of integrating decentralized exchange (DEX) functionalities according to the requirements specified by jup.ag.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Integration with jup.ag](#integration-with-jupag)
- [Related Repositories](#related-repositories)
- [Contributing](#contributing)
- [License](#license)

## Overview

The Swap.io CLMM Rust SDK enables developers to interact with the underlying Swap.io CLMM smart contracts, facilitating smooth integration with the jup.ag swap router program. The SDK abstracts many of the low-level details, allowing you to focus on building robust applications on top of decentralized liquidity pools.

## Features

- **DEX Integration:** Seamless integration with the jup.ag swap router as per the [integration documentation](https://station.jup.ag/docs/dex-integration).
- **Rust Native:** Built using Rust, offering strong performance and memory safety.
- **Abstraction Layer:** Simplified API for interacting with the Swap.io CLMM smart contracts.
- **Extensible:** Designed with modularity in mind to support future features and integrations.

## Installation

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system. Then, clone the repository and build the project using Cargo:

```bash
git clone https://github.com/swap-dot-io/swap-io-clmm-rust-sdk.git
cd swap-io-clmm-rust-sdk
cargo build --release
```

## Usage

Below is a brief example to demonstrate how to initialize the SDK and interact with the swap.io clmm smart contracts:

```rust
use swap_io_clmm_sdk::{Client, Config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure the SDK (update with appropriate endpoint or key as needed)
    let config = Config::new("https://api.swap-io.io");
    let client = Client::new(config);

    // Example: Query liquidity pool information
    let pool_info = client.get_pool_info("POOL_ID")?;
    println!("Pool Info: {:?}", pool_info);

    // Example: Create a swap quote using jup.ag integration
    let swap_quote = client.get_swap_quote("TOKEN_A", "TOKEN_B", 1000)?;
    println!("Swap Quote: {:?}", swap_quote);

    Ok(())
}
```

For more detailed examples and API documentation, please refer to the project's documentation.

## Integration with jup.ag

This SDK has been developed to meet the integration requirements specified in the [jup.ag swap router program documentation](https://station.jup.ag/docs/dex-integration). The SDK handles:

- Data formatting and communication with the jup.ag endpoint.
- Managing request/response cycles for swap quotes.
- Ensuring compatibility with the required protocol standards for DEX integrations.

Be sure to consult the jup.ag documentation for further details on configuring and using the swap router features.

## Related Repositories

- **Swap.io CLMM Smart Contracts:** The original smart contract implementation can be found in the [swap.io clmm repository](https://github.com/swap-dot-io/swap-io-clmm/).
- **jup.ag Documentation:** Detailed instructions and requirements are available at [jup.ag docs](https://station.jup.ag/docs/dex-integration).

## Contributing

Contributions to the SDK are welcome! Please review the [contributing guidelines](CONTRIBUTING.md) before submitting a pull request. For major changes, open an issue first to discuss what you would like to change.

## License

This project is licensed under the [MIT License](LICENSE).






