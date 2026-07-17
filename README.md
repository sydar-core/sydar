## About us

Sydar — a sovereign digital money designed as a resilient, auditable, and highly accessible settlement layer.  Sydar is a peer-to-peer monetary system built to enable trust-minimized value transfer at global scale while preserving simplicity, predictability, and broad participation. While avoiding the severe state-bloat of general-purpose Turing-complete smart contracts, the protocol focuses on secure, low-friction digital money coupled with native Web5 Decentralized Identity (DID) primitives.

 Sydar is implemented on  SydarDAG, a directed acyclic graph (DAG) data structure that permits parallel block creation by independent miners. Unlike traditional single-chain blockchains that discard concurrent blocks as orphans,  SydarDAG embraces them: every valid block contributes to the ledger, and the  Sydar Consensus protocol produces a single deterministic total ordering of all transactions.

This eliminates the wasted-hash problem that plagues single-chain designs. The network is secured by  SydarX — an application-layer Proof-of-Work combining Blake3 (super-speed hashing) with an 8-stage XOR memory loop requiring 16 MB of random-access memory per hash operation. This architecture makes  SydarX genuinely memory-hard, compressing the performance gap between ASICs and general-purpose hardware (GPUs, CPUs), ensuring mining remains accessible to a broad population of participants.

 Sydar operates on a hybrid Account + Object state model. Rather than tracking individual unspent outputs (UTXOs), the protocol maintains per-address balances and nonces. Each transaction atomically deducts from the sender and credits the receiver, with balance finality enforced at the block confirmation boundary. This model delivers account + object based ergonomics with the security of Proof-of-Work.

## Key Features
 * **High Throughput:** Engineered to handle up to 10,000+ TPS.
 * **Web5 Native:** Built-in support for Decentralized Identifiers (DIDs) and DWN synchronization.
 * **Sovereign Infrastructure:** 21 Million hard-capped supply with a fair-launch, no-premine model.
 * **Quantum Ready:** Future-proof architecture designed for post-quantum security transitions.
## The Genesis Phase
> **Note:** Our network architecture is designed for 1-second finality. We are constantly optimizing the DAG topology to reach our 10,000 TPS milestone on consumer-grade hardware.
> 
## Web5 Identity Integration
Unlike traditional L1s, sydar nodes are natively compatible with Decentralized Web Nodes (DWN). 
Every miner and user can link their on-chain CSM address with a W3C-compliant DID (Decentralized Identifier). 
This allows for secure, serverless messaging and data storage directly on the sydar infrastructure.

## Installation
### Building from Source (Recommended)
Building from source ensures you are running the most optimized version for your specific hardware.
**Prerequisites (Linux/Ubuntu):**
```bash
sudo apt update && sudo apt install -y git build-essential cmake pkg-config libssl-dev

```
**Build Commands:**
```bash
# Clone the repository
git clone https://github.com/sydar-core/sydar.git
cd sydar

# Compile the node and miner
cargo build --release

```
##  Running the Node
### Start a Mainnet Node
To start a full-node and begin syncing with the sydar DAG:
```bash
./target/release/sydard --utxoindex

```
### Start a Testnet Node
For developers looking to test integrations or mining setups:
```bash
./target/release/sydard --testnet

```
## Mining on sydar
sydar uses a memory-hard, ASIC-resistant PoW algorithm. To start mining with your CSM address:
```bash
cd sydar-miner
./target/release/sydar-miner -s 127.0.0.1 -p 26110 -a <YOUR_CSM_ADDRESS> --mine-when-not-synced

```
## wRPC & Integration
sydar provides a high-performance **wRPC (WebSocket RPC)** interface for exchanges, wallets, and explorers.
 * **Default Port:** 26110

## Contributing
We welcome contributions to the sydar core! Whether it's optimizing the Rust codebase, improving the DAG consensus, or enhancing Web5 integration, your help is vital.
 1. Fork the repo.
 2. Create your feature branch (git checkout -b feature/amazing-feature).
 3. Commit your changes.
 4. Push to the branch.
 5. Open a Pull Request.
