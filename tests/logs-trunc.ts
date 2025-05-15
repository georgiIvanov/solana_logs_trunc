import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LogsTrunc } from "../target/types/logs_trunc";
import { PublicKey } from '@solana/web3.js';

describe("logs-trunc", () => {
  // Configure the client to use devnet
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Use your deployed program ID
  const programId = new PublicKey("DUKnRfntDqsg2jvN5JUvh8otaCAfQe4Q5etkrdm8tE4D");
  
  // Initialize the program with your deployed program ID
  const program = new anchor.Program(
    require("../target/idl/logs_trunc.json"),
    provider
  ) as Program<LogsTrunc>;

  it("Is initialized!", async () => {
    const tx = await program.methods.initialize().rpc();
    console.log("Initialize transaction signature", tx);
  });

  it("Calls deposit function and emits event", async () => {
    try {
      // Call the deposit function
      const tx = await program.methods.deposit().rpc();
      console.log("Deposit transaction signature", tx);
      
      // If you want to fetch and examine transaction logs
      const txDetails = await provider.connection.getTransaction(tx, {
        commitment: "confirmed",
      });
      console.log("Transaction logs:", txDetails?.meta?.logMessages);
      
    } catch (error) {
      console.error("Error:", error);
      throw error;
    }
  });
});