import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Pluto } from "../target/types/pluto";
// @ts-ignore
import accs from "./accounts.json";

describe("leverage-create-pyusd", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();

  const accounts = accs.dev.pyusd;

  const program = anchor.workspace.Pluto as Program<Pluto>;

  it("Is initialized!", async () => {
    let usdcFeed = Array.from(Uint8Array.from(Buffer.from(accounts.tokenCollateralPriceFeed, "utf8")));
    let jlpFeed = Array.from(Uint8Array.from(Buffer.from(accounts.nativeCollateralPriceFeed, "utf8")));
    // Add your test here.
    const ix = await program.methods.leverageVaultCreate(
        usdcFeed, jlpFeed,
    ).accounts({
      leverageConfig: accounts.leverageConfig,
      tokenCollateralTokenProgram: new anchor.web3.PublicKey(accounts.tokenProgramA),
      tokenCollateralTokenMint: new anchor.web3.PublicKey(accounts.tokenMintA),
      tokenCollateralPriceOracle: new anchor.web3.PublicKey(accounts.tokenCollateralPriceOracle),
      nativeCollateralTokenProgram: new anchor.web3.PublicKey(accounts.tokenProgramB),
      nativeCollateralTokenMint: new anchor.web3.PublicKey(accounts.tokenMintB),
      nativeCollateralPriceOracle: new anchor.web3.PublicKey(accounts.nativeCollateralPriceOracle),
    }).instruction();

    const liq_ix = await program.methods.leverageVaultCreateLiquidity().accounts({
      leverageConfig: accounts.leverageConfig,
      tokenCollateralTokenProgram: new anchor.web3.PublicKey(accounts.tokenProgramA),
      tokenCollateralTokenMint: new anchor.web3.PublicKey(accounts.tokenMintA),
      nativeCollateralTokenProgram: new anchor.web3.PublicKey(accounts.tokenProgramB),
      nativeCollateralTokenMint: new anchor.web3.PublicKey(accounts.tokenMintB),
    }).instruction();

    let transaction = new anchor.web3.Transaction();

    transaction.add(
      anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
        units: 500000
      }),
      anchor.web3.ComputeBudgetProgram.setComputeUnitPrice({
        microLamports: 0
      }),
    );

    transaction.add(
      ix,
      liq_ix,
    );

    let tx = await provider.sendAndConfirm(transaction);

    console.log("Your transaction signature", tx);
  });
});