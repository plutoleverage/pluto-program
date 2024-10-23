import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Pluto } from "../target/types/pluto";
// @ts-ignore
import accs from "./accounts.json";

describe("pluto", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();

  const program = anchor.workspace.Pluto as Program<Pluto>;

  let accounts = accs.staging;

  it("Is initialized!", async () => {
    let usdcFeed = Array.from(Uint8Array.from(Buffer.from(accounts.tokenCollateralPriceFeed, "hex")));
    // Add your test here.
    const ix = await program.methods.earnVaultChangePriceOracle(
        usdcFeed,
    ).accounts({
      vault: new anchor.web3.PublicKey(accounts.earnVault),
    }).instruction();

    let transaction = new anchor.web3.Transaction();

    transaction.add(
        anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
          units: 200000
        }),
        anchor.web3.ComputeBudgetProgram.setComputeUnitPrice({
          microLamports: 1000
        })
    );

    transaction.add(ix);

    console.log("usdcFeed", usdcFeed, Array.from(usdcFeed), Uint8Array.from(usdcFeed));

    let tx = await provider.sendAndConfirm(transaction, [],{
        skipPreflight: true,
    });

    console.log("Your transaction signature", tx);
    console.log("Your transaction", transaction);
  });
});
