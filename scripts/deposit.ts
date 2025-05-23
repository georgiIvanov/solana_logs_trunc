import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LogsTrunc } from "../target/types/logs_trunc";
import { PublicKey } from '@solana/web3.js';
import * as fs from 'fs';
import * as toml from 'toml';
import * as path from 'path';
import BN from "bn.js";

// Read program IDs from Anchor.toml
const configPath = path.resolve(__dirname, '../Anchor.toml');
const configFile = fs.readFileSync(configPath, 'utf8');
const config = toml.parse(configFile);

console.log(`config: `, config);

if (!process.env.ANCHOR_PROVIDER_URL) {
  process.env.ANCHOR_PROVIDER_URL = "https://api.devnet.solana.com";
}

if (!process.env.ANCHOR_WALLET) {
  process.env.ANCHOR_WALLET = require("os").homedir() + "/.config/solana/id.json";
}

// Get program IDs from config (use devnet)
const logsTruncId = new PublicKey(config.programs.devnet.logs_trunc);
const emitterId = new PublicKey(config.programs.devnet.emitter);

console.log(`LogsTrunc program ID: ${logsTruncId.toString()}`);
console.log(`Emitter program ID: ${emitterId.toString()}`);

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = new anchor.Program(
  require("../target/idl/logs_trunc.json"),
  provider
) as Program<LogsTrunc>;

async function executeDeposit() {
  try {
    console.log("Calling deposit with moderate parameters...");
    // CPI calls x row_lengths x row_count
    //  1 cpi    x 100 chars   x 5         = 500 bytes
    // 20 cpi    x 100 chars   x 5         = 10000 bytes = 10 kb
    const tx = await program.methods
      .deposit(new BN(20), new BN(100), new BN(5))
      .accounts({
        user: provider.wallet.publicKey,
      })
      .rpc();
    
    console.log("Deposit transaction signature:", tx);
    
    // Try to fetch the transaction details
    try {
      const txDetails = await provider.connection.getTransaction(tx, {
        commitment: "confirmed",
      });
      
      if (txDetails?.meta?.logMessages) {
        console.log("First few transaction logs:", 
          txDetails.meta.logMessages.slice(0, 5));
        console.log(`...and ${txDetails.meta.logMessages.length - 5} more log messages`);
      }
    } catch (fetchError) {
      console.log("Could not fetch transaction details:", fetchError);
    }
    
  } catch (error) {
    console.error("Error executing deposit:", error);
    
    // Try to extract logs from error
    if (error instanceof Error) {
      const errorString = error.toString();
      console.log("Error message:", errorString);
      
      // Try to extract any logs from the error message
      const logsMatch = errorString.match(/Logs:\s*\[([\s\S]*)\]/);
      if (logsMatch && logsMatch[1]) {
        console.log("Extracted logs from error:");
        console.log(logsMatch[1]);
      }
    }
  }
}

// Execute the function
executeDeposit().then(
  () => process.exit(0),
  (err) => {
    console.error(err);
    process.exit(1);
  }
);