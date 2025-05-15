# Run PoC

1. `npm install`
2. `anchor run deposit`
3. Look up logs in the tx signature

# To deploy on devnet

1. `solana airdrop 2 --url devnet`
2. `anchor build`
3. `anchor test` - `provider.cluster` must be `devnet`

# To run tests on localnet

Change `provider.cluster` to `localnet` (in `Anchor.toml`)

## Errors

Some common errors that were fixed during making of this PoC


```bash
Deploying cluster: https://api.devnet.solana.com
Upgrade authority: /Users/georgiivanov/.config/solana/id.json
Deploying program "emitter"...
Program path: /Users/georgiivanov/Projects/web3/solana/logs-trunc/target/deploy/emitter.so...
Error: Error processing Instruction 0: account data too small for instruction
There was a problem deploying: Output { status: ExitStatus(unix_wait_status(256)), stdout: "", stderr: "" }.
```

Fix: [Reallocate](https://stackoverflow.com/questions/71267943/solana-deploy-account-data-too-small-for-instruction) more bytes for program
`solana program extend <PROGRAM_ID> <MORE_BYTES>`

Example:
```bash
logs-trunc % solana program extend HVxkVckuk55bkXeXZoz78aMsVYL5ChTyQ8aiPymicUTf 20000 -u d -k ~/.config/solana/id.json

Extended Program Id HVxkVckuk55bkXeXZoz78aMsVYL5ChTyQ8aiPymicUTf by 1000 bytes
```

