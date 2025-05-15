import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LogsTrunc } from "../target/types/logs_trunc";
import { Keypair, PublicKey } from '@solana/web3.js';
import { assert } from "chai";
import BN from "bn.js";


describe("logs-trunc", () => {
  // Configure the client to use devnet
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const programId = new PublicKey("DUKnRfntDqsg2jvN5JUvh8otaCAfQe4Q5etkrdm8tE4D");

  const [counterPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("counter")],
    programId
  );


  // Initialize the program with your deployed program ID
  const program = new anchor.Program(
    require("../target/idl/logs_trunc.json"),
    provider
  ) as Program<LogsTrunc>;

  // it("Is initialized!", async () => {
  //   const tx = await program.methods
  //     .initialize()
  //     .accounts({
  //       user: provider.wallet.publicKey,
  //     })
  //     .rpc();
      
  //   console.log("Initialize transaction signature", tx);
  //   console.log("Counter PDA:", counterPDA.toString());

  //   // Verify the counter was initialized properly
  //   const counterAccount = await program.account.counter.fetch(counterPDA);
  //   assert.equal(counterAccount.value.toNumber(), 0);
  // });

  it("Calls deposit function and emits event", async () => {
    try {
      // Call the deposit function with the counter PDA
      const tx = await program.methods
        .deposit(new BN(1000), new BN(50))
        .accounts({
          counter: counterPDA,
        })
        .rpc();
      
      console.log("Deposit transaction signature", tx);
      
      // If you want transaction logs (note: may be truncated due to large messages)
      const txDetails = await provider.connection.getTransaction(tx, {
        commitment: "confirmed", maxSupportedTransactionVersion: 0
      });

      if (txDetails?.meta?.logMessages) {
        // Only log first few messages to avoid console overflow
        console.log("First few transaction logs:", 
          txDetails.meta.logMessages.slice(0, 3));
        console.log(`...and ${txDetails.meta.logMessages.length - 3} more log messages`);
      }
      
    } catch (error) {
      console.error("Error:", error);
      throw error;
    }
  });
});