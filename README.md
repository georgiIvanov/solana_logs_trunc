# Run PoC

1. `npm install`
2. `anchor run deposit`
3. Look up logs in the tx signature

# To deploy on devnet

1. `solana airdrop 2 --url devnet`
2. `anchor build`
3. `anchor test` - `Anchor.toml::provider.cluster` must be `devnet`

# To run tests on localnet

Change `provider.cluster` to `localnet` (in `Anchor.toml`)

