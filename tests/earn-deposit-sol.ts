import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Pluto } from "../target/types/pluto";
import { PublicKey } from "@solana/web3.js";
// @ts-ignore
import accs from "./accounts.json";

describe("deposit-sol", () => {
  const provider = anchor.AnchorProvider.env();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const accounts = accs.dev.sol;
  const vaultAccount = new PublicKey(accounts.earnVault);
  const program = anchor.workspace.Pluto as Program<Pluto>;

  it("Is initialized!", async () => {
    // Add your test here.
    const wrap_sol_ix = await program.methods.wrapSol(new anchor.BN(1*1e7)).instruction();

    const deposit_ix = await program.methods.earnVaultDeposit(new anchor.BN(1*1e7)).accounts({
      vault: vaultAccount,
    }).instruction();

    let trx = new anchor.web3.Transaction();
    trx.add(
        anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
          units: 300000,
        }),
        anchor.web3.ComputeBudgetProgram.setComputeUnitPrice({
          microLamports: 0,
        }),
        wrap_sol_ix,
        deposit_ix,
    );

    let tx = await provider.sendAndConfirm(trx);

    console.log(`TX: ${tx}`);
  });
});