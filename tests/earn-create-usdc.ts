import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
import { Pluto } from "../target/types/pluto";
// @ts-ignore
import accs from "./accounts.json";
import {PublicKey} from "@solana/web3.js";

describe("earn-create-usdc", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const accounts = accs.dev.usdc;

  const tokenMint = new anchor.web3.PublicKey(accounts.tokenMintA);
  const tokenCollateralPriceOracle = new anchor.web3.PublicKey(accounts.tokenCollateralPriceOracle);
  let usdcFeed = Array.from(Uint8Array.from(Buffer.from(accounts.tokenCollateralPriceFeed, "utf8")));

  const program = anchor.workspace.Pluto as Program<Pluto>;

  it("Is initialized!", async () => {
    // Add your test here.
    // const tx = await program.methods.createVault().rpc();
    const tx = await program.methods.earnVaultCreate(usdcFeed).accounts({
      earnConfig: accounts.earnConfig,
      tokenMint: tokenMint,
      tokenProgram: new PublicKey(accounts.tokenProgramA),
      priceOracle: tokenCollateralPriceOracle,
    }).preInstructions([
      anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
        units: 300000,
      }),
    ]).rpc({
      skipPreflight: false
    }).catch(e => console.error(e));

    console.log("Your transaction signature", tx);
  });
});