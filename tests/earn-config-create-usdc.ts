import * as anchor from "@coral-xyz/anchor";
import {Program, web3} from "@coral-xyz/anchor";
import { Pluto } from "../target/types/pluto";
import {PublicKey} from "@solana/web3.js";
// @ts-ignore
import accs from "./accounts.json";

describe("earn-config-create-usdc", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  let provider = anchor.getProvider();
  const accounts = accs.dev.usdc;
  const program = anchor.workspace.Pluto as Program<Pluto>;

  it("Is initialized!", async () => {
    // Add your test here.
    // const tx = await program.methods.createVault().rpc();
    const tx = await program.methods.earnConfigCreate(
        false, // Frozen
        2000, // Protocol Fee (2%)
        90000, // LTV (90%)
        0, // Deposit Fee (0%)
        new anchor.BN(1).mul(new anchor.BN(10).pow(new anchor.BN(accounts.tokenDecimalA))), // Min Deposit Limit (0)
        new anchor.BN(1000000).mul(new anchor.BN(10).pow(new anchor.BN(accounts.tokenDecimalA))), // Max Deposit Limit (1,000,000)
        0, // Withdraw Fee (0%)
        new anchor.BN(1), // Min Withdraw Limit (0)
        new anchor.BN(1000000).mul(new anchor.BN(10).pow(new anchor.BN(accounts.tokenDecimalA))), // Max Withdraw Limit (1,000,000)
        0, // Borrow Fee (0%)
        new anchor.BN(1), // Min Borrow Limit (0)
        new anchor.BN(100000).mul(new anchor.BN(10).pow(new anchor.BN(accounts.tokenDecimalA))), // Max Borrow Limit (1,000,000)
        64000 // Floor Cap (64%)
    ).accounts({
      protocol: accounts.protocol,
      indexer: accounts.indexer,
      feeVault: accounts.feeVault,
      tokenMint: accounts.tokenMintA
    }).rpc({
        skipPreflight:false
    }).catch(e => console.error(e));

    console.log("Your transaction signature", tx);
  });
});