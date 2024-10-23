import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Pluto } from "../target/types/pluto";
import { PublicKey } from "@solana/web3.js";
// @ts-ignore
import accs from "./accounts.json";

describe("earn-withdraw-pyusd", () => {
  const provider = anchor.AnchorProvider.env();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  const accounts = accs.dev.pyusd;
  const program = anchor.workspace.Pluto as Program<Pluto>;

  it("Is initialized!", async () => {
    // Add your test here.
    const vaultAccount = new PublicKey(accounts.earnVault);

    const withdraw_ix = await program.methods.earnVaultWithdraw(new anchor.BN(1*1e8), new anchor.BN(1*1e6)).accounts({
      vault: vaultAccount,
    }).instruction();

    let trx = new anchor.web3.Transaction();
    trx.add(
        anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
          units: 500000
        }),
        anchor.web3.ComputeBudgetProgram.setComputeUnitPrice({
          microLamports: 0,
        }),
        withdraw_ix,
    );

    let tx = await provider.sendAndConfirm(trx,[],{skipPreflight: true});

    console.log(`TX: ${tx}`);
  });
});