import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Pluto } from "../target/types/pluto";
import { PublicKey } from "@solana/web3.js";
// @ts-ignore
import accs from "./accounts.json";

describe("earn-withdraw-sol", () => {
  const provider = anchor.AnchorProvider.env();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  const accounts = accs.dev.sol;
  const program = anchor.workspace.Pluto as Program<Pluto>;

  it("Is initialized!", async () => {
    // Add your test here.
    const vaultAccount = new PublicKey(accounts.earnVault);

    const withdraw_ix = await program.methods.earnVaultWithdraw(new anchor.BN(1*1e6), new anchor.BN(1*1e7)).accounts({
      vault: vaultAccount,
    }).instruction();

    const unwrap_sol_ix = await program.methods.unwrapSol(new anchor.BN(1*1e7)).instruction();

    let trx = new anchor.web3.Transaction();
    trx.add(
        anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
          units: 500000
        }),
        anchor.web3.ComputeBudgetProgram.setComputeUnitPrice({
          microLamports: 0,
        }),
        withdraw_ix,
        unwrap_sol_ix,
    );

    let tx = await provider.sendAndConfirm(trx, [], {skipPreflight: true});

    console.log(`TX: ${tx}`);
  });
});