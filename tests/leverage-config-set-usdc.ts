import * as anchor from "@coral-xyz/anchor";
import {Program, web3} from "@coral-xyz/anchor";
import { Pluto } from "../target/types/pluto";
import {PublicKey} from "@solana/web3.js";
// @ts-ignore
import accs from "./accounts.json";

describe("leverage-config-set-usdc", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const accounts = accs.dev.usdc;
  const program = anchor.workspace.Pluto as Program<Pluto>;

  it("Is initialized!", async () => {
    // Add your test here.
    // const tx = await program.methods.createVault().rpc();
    const tx = await program.methods.leverageConfigSet(
        false, // Frozen
        2000, // Protocol Fee (2%)
        1100, // Min Leverage (1.1x)
        9500, // Max Leverage (9.5x)
        100, // Leverage Step (0.1x)
        0, // Leverage Fee (0%)
        new anchor.BN(1).mul(new anchor.BN(10).pow(new anchor.BN(accounts.tokenDecimalA))), // Min Leverage Limit (1)
        new anchor.BN(100000).mul(new anchor.BN(10).pow(new anchor.BN(accounts.tokenDecimalA))), // Max Leverage Limit (1,000,000)
        0, // Deleverage Fee (0%)
        new anchor.BN(1), // Min Deleverage Limit (1)
        new anchor.BN(100000).mul(new anchor.BN(10).pow(new anchor.BN(accounts.tokenDecimalB))), // Max Deleverage Limit (1,000,000)
        0, // Closing Fee (0%)
        2500, // Spread Rate (2.5%)
        2500, // Liquidation Fee (2.5%)
        95000, // Liquidation Threshold (95%)
        0, // Liquidation Protocol Ratio (0%)
        500, // Slippage Rate (0.5%)
        new anchor.BN(2 * 86400), // Emergency Eject Duration (2 days)
        1050, // Saver threshold 1.05 health factor
        500, // Saver Target 0.5x leverage
    ).accounts({
      config: accounts.leverageConfig,
      feeVault: accounts.feeVault,
    }).rpc({
        skipPreflight:false
    }).catch(e => console.error(e));

    console.log("Your transaction signature", tx);
  });
});
