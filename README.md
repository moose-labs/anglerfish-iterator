# Anglerfish Iterator

**Anglerfish Iterator** is a Rust-based application that interacts with the Sui network to iterate through the global state of lottery phases in the Anglerfish Protocol. The program automates the process of fetching the current phase, waiting for its completion, and transitioning to the next phase.

## Features

The application supports the following phases in the Anglerfish Protocol:

1. **Liquidity Providing**: Enables users to provide liquidity for the pool.
2. **Ticketing**: Manages ticket sales for participants.
3. **Drawing**: Conducts the lottery draw.
4. **Distributing**: Distributes rewards to winners.
5. **Settling**: Finalizes and resets the pool then create the new round.

## Configuration

Before running the application, configure the required addresses and capabilities in the `anglerfish_iterator_config.toml` file. The configuration file should look like this:

```toml
package_id = "0x09c23dc75590103509b266b5f54fa38e73313b5ec9ddba480951efd9c70bec00"

[objects]
phase_info_id = "0x7b5e50c18c4fde1c342d6a2920e24d833e41ac1c497fa0ac06b6f2d46fc959ee"
round_registry_id = "0x40601860c44486096f4a8704b07601fbfa9f4bbe3126a64fe192fec9ab29c8cd"
pool_registry_id = "0x0e1e5527da787ffa1665e94e9eec5dbcd422aa5bcfd8c62f45cafd40c745bc98"
prize_pool_id = "0x114580221e43cada6daba9c93b7370ea9ff0ddb83f9c7a8d6cd962add4212efd"
lounge_registry_id = "0x7ed2df7a7ee900f4deec82211464babf2dc568185d332371bcabe63def09b7a5"

[pool]
coin_type = "0xc51004215439bd6a6acd47e5fd128b264203cba1059afd5374341e5d850326fc::usdc::USDC"

[iterator]
cap_id = "0xb1d079bb4b76d5bfadd49c94ff8b5e6e66eb60cc2a7c68b4e832938298c06325"
```

Key Fields:

- package_id: The ID of the Anglerfish Protocol package.
- phase_info_id: The object ID that stores the current phase information.
- round_registry_id: The ID for the round registry.
- pool_registry_id: The ID for the pool registry.
- prize_pool_id: The object ID of the prize pool.
- lounge_registry_id: The ID for the lounge registry.
- coin_type: The coin type used in the pool (e.g., USDC).
- cap_id: The capability ID required for operations.

## Installation

Ensure you have Rust installed on your system. Then, clone this repository and navigate to the project directory:

```bash
git clone https://github.com/moose-labs/anglerfish-iterator.git
cd anglerfish-iterator
```

The application will continuously fetch the current phase, wait for its completion, and proceed to the next phase.

## License

This project is licensed under a custom license.  
**Usage Restrictions:**

- You are free to read and study the code.
- Commercial usage or use in competitive projects is **strictly prohibited** without prior written permission from the project owner.
