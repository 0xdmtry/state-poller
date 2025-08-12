# Fuel On-Chain State Poller

A backend service built in Rust that monitors a smart contract on the Fuel blockchain. The service periodically polls a `u64` value from a live Sway contract and prints it to the console. This project serves as a foundation for building more complex off-chain bots, indexers, and services that react to on-chain state.

***

## Architecture

This project consists of two main components:
* **Sway Smart Contract:** A simple "Greeter" contract deployed on the Fuel testnet that stores a `u64` value and has functions to `set_value` and `get_value`.
* **Rust Polling Service:** A long-running [Tokio](https://tokio.rs/)-based application that connects to a Fuel node, instantiates the contract, and polls its state in a loop.

***

## ⚙️ Setup & Installation

The setup process involves compiling and deploying the on-chain contract, then configuring the off-chain service.

#### 1. Compile the Smart Contract
The Sway contract is compiled using `forc`. This generates the contract's ABI file, which is needed by the Rust service.

```bash
cd contracts/greeter
forc build
```

#### 2. Deploy the Smart Contract
The compiled contract is deployed to the testnet to get its live address.

```bash
forc deploy --testnet
```
The **`Contract id`** from the output is required for the next step.

#### 3. Configure the Rust Service
Navigate back to the project's root directory.

1.  A `.env` file must be created in the root of the project.
2.  The following variables must be added to the `.env` file, populated with the contract ID from the previous step and the private key of a funded testnet wallet.

    ```env
    FUEL_NODE_URL=https://testnet.fuel.network/v1/graphql
    GREETER_CONTRACT_ID=0x...
    PRIVATE_KEY=0x...
    ```

***

## Usage

Running the system requires two separate terminals.

#### Terminal 1: Start the Polling Service
From the project root, the Rust application is started.

```bash
cargo run
```

The service will start and begin polling the `get_value` function on the deployed contract, printing the current value every 5 seconds.
```
Polling contract state every 5 seconds...
Contract's current value: 0
Contract's current value: 0
...
```

#### Terminal 2: Change the Contract's State
To test the poller, the value stored in the contract can be changed using the `forc call` command.

Replace `<CONTRACT_ID>` and `<PRIVATE_KEY>` with the relevant values.

```bash
forc call \
--testnet \
--signing-key <PRIVATE_KEY> \
<CONTRACT_ID> \
set_value 42 \
--mode live
```

#### Observe the Result
After the `forc call` transaction succeeds, the output in Terminal 1 will change to reflect the new value on its next poll.

```
Polling contract state every 5 seconds...
Contract's current value: 0
Contract's current value: 0
Contract's current value: 42  <-- Success!
Contract's current value: 42
...
