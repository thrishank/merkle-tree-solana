# Merkle Tree Implementation on Solana

Storing thousands of accounts directly on Solana is expensive due to rent requirements.

For example, storing **1000 addresses** (each 32 bytes) costs roughly **0.22 SOL** in rent:

```bash
> 1000 * 32
32000
> solana rent 32000
Rent-exempt minimum: 0.22361088 SOL
```

A better solution is to use a Merkle Tree.

Instead of storing every address on-chain, we only store a single 32-byte Merkle root hash.
To verify that an address is included, we provide a Merkle proof: the list of sibling hashes that allow the program to recompute the root from the leaf node.

This saves substantial on-chain storage while still allowing secure inclusion verification.
