
This is a demo on Solana program log truncation when logs reach 10kb.

# Run PoC

1. `npm install`
2. `anchor build`
3. `anchor run deposit`
4. Look up logs using the `Deposit transaction signature: ...`

# To deploy on devnet

1. `solana airdrop 2 --url devnet`
2. `anchor build`
3. `anchor test` - `Anchor.toml::provider.cluster` must be `devnet`

# To run tests on localnet

Change `provider.cluster` to `localnet` (in `Anchor.toml`)

# Examples

A deposit function is invoked. At the end it emits a `Deposit Executed` event log.
In one case the deposit is visible since few logs were printed and in the other its not.

- [Full logs](https://explorer.solana.com/tx/5n2h8xvpZdsZxfdzvczP14jY36Wtbe26hDZD8ULmrayD22Y5chYCqRB1M829KXdnHYmxzpgiDtdLWD59ch8rzPmX?cluster=devnet)
- [Truncated logs](https://explorer.solana.com/tx/5hNNQ6vJFfSZC7TckyJtGjV1TguQXcm4coiC4PaNGcnra86mPdB49uFAcw24qxobftXasvs6v7dc5uDncjnQELSV?cluster=devnet)