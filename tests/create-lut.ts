
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Pluto } from "../target/types/pluto";
import {PublicKey} from "@solana/web3.js";
// @ts-ignore
import accs from './accounts.json';

const provider = anchor.AnchorProvider.env()
anchor.setProvider(provider);

/*describe("create-lut-usdc", () => {
  // Configure the client to use the local cluster.
  const accounts = accs.dev.usdc;

  const program = anchor.workspace.Pluto as Program<Pluto>;

  let lookupTablePubkey: PublicKey;

  it("Create an Address Lookup Table", async () => {
    let ix: anchor.web3.TransactionInstruction;
    [ix, lookupTablePubkey] = anchor.web3.AddressLookupTableProgram.createLookupTable({
      authority: provider.wallet.publicKey,
      payer: provider.wallet.publicKey,
      recentSlot: await provider.connection.getSlot(),
    });

    await sendTransactionV0([ix]);

    console.log("Pubkeys from generated keypairs:");
    console.log(`LUT Account #1: ${provider.wallet.publicKey}`);
    console.log(`LUT Account Addr #1: ${lookupTablePubkey}`);

    ix = anchor.web3.AddressLookupTableProgram.extendLookupTable({
      addresses: [
        new PublicKey(accounts.protocol),
        new PublicKey(accounts.tokenProgramA),
        new PublicKey(accounts.tokenMintA),
        new PublicKey(accounts.tokenProgramB),
        new PublicKey(accounts.tokenMintB),
        new PublicKey(accounts.tokenCollateralPriceOracle),
        new PublicKey(accounts.nativeCollateralPriceOracle),
        new PublicKey(accounts.earnConfig),
        new PublicKey(accounts.earnStats),
        new PublicKey(accounts.earnVaultAuthority),
        new PublicKey(accounts.earnVault),
        new PublicKey(accounts.earnVaultLiquidity),
        new PublicKey(accounts.leverageConfig),
        new PublicKey(accounts.leverageStats),
        new PublicKey(accounts.leverageVaultAuthority),
        new PublicKey(accounts.leverageVault),
        new PublicKey(accounts.leverageVaultTokenCollateralLiquidity),
        new PublicKey(accounts.leverageVaultNativeCollateralLiquidity),
      ],
      authority: provider.wallet.publicKey,
      lookupTable: new PublicKey(lookupTablePubkey),
      payer: provider.wallet.publicKey,
    });

    await sendTransactionV0([ix]);

    await printAddressLookupTable(lookupTablePubkey);
  });
});*/

/*describe("extend-lut-usdc", () => {
  // Configure the client to use the local cluster.
  const accounts = accs.dev.usdc;

  const program = anchor.workspace.Pluto as Program<Pluto>;

  let lookupTablePubkey: PublicKey;

  it("Create an Address Lookup Table", async () => {
    let ix: anchor.web3.TransactionInstruction;
    let lookupTablePubkey = new PublicKey(accounts.lookupTable);

    ix = anchor.web3.AddressLookupTableProgram.extendLookupTable({
      addresses: [
        new PublicKey(accounts.earnStats),
        new PublicKey(accounts.leverageStats),
      ],
      authority: provider.wallet.publicKey,
      lookupTable: new PublicKey(lookupTablePubkey),
      payer: provider.wallet.publicKey,
    });

    await sendTransactionV0([ix]);

    await printAddressLookupTable(lookupTablePubkey);
  });
});*/

/*describe("create-lut-pyusd", () => {
  // Configure the client to use the local cluster.
  const accounts = accs.dev.pyusd;

  const program = anchor.workspace.Pluto as Program<Pluto>;

  let lookupTablePubkey: PublicKey;

  it("Create an Address Lookup Table", async () => {
    let ix: anchor.web3.TransactionInstruction;
    [ix, lookupTablePubkey] = anchor.web3.AddressLookupTableProgram.createLookupTable({
      authority: provider.wallet.publicKey,
      payer: provider.wallet.publicKey,
      recentSlot: await provider.connection.getSlot(),
    });

    await sendTransactionV0([ix]);

    console.log("Pubkeys from generated keypairs:");
    console.log(`LUT Account #1: ${provider.wallet.publicKey}`);
    console.log(`LUT Account Addr #1: ${lookupTablePubkey}`);

    ix = anchor.web3.AddressLookupTableProgram.extendLookupTable({
        addresses: [
          new PublicKey(accounts.protocol),
          new PublicKey(accounts.tokenProgramA),
          new PublicKey(accounts.tokenMintA),
          new PublicKey(accounts.tokenProgramB),
          new PublicKey(accounts.tokenMintB),
          new PublicKey(accounts.tokenCollateralPriceOracle),
          new PublicKey(accounts.nativeCollateralPriceOracle),
          new PublicKey(accounts.earnConfig),
          new PublicKey(accounts.earnStats),
          new PublicKey(accounts.earnVaultAuthority),
          new PublicKey(accounts.earnVault),
          new PublicKey(accounts.earnVaultLiquidity),
          new PublicKey(accounts.leverageConfig),
          new PublicKey(accounts.leverageStats),
          new PublicKey(accounts.leverageVaultAuthority),
          new PublicKey(accounts.leverageVault),
          new PublicKey(accounts.leverageVaultTokenCollateralLiquidity),
          new PublicKey(accounts.leverageVaultNativeCollateralLiquidity),
        ],
        authority: provider.wallet.publicKey,
        lookupTable: new PublicKey(lookupTablePubkey),
        payer: provider.wallet.publicKey,
      });

      await sendTransactionV0([ix]);

      await printAddressLookupTable(lookupTablePubkey);
  });
});*/

/*describe("extend-lut-pyusd", () => {
  // Configure the client to use the local cluster.
  const accounts = accs.dev.pyusd;

  const program = anchor.workspace.Pluto as Program<Pluto>;

  let lookupTablePubkey: PublicKey;

  it("Create an Address Lookup Table", async () => {
    let ix: anchor.web3.TransactionInstruction;
    let lookupTablePubkey = new PublicKey(accounts.lookupTable);

    ix = anchor.web3.AddressLookupTableProgram.extendLookupTable({
      addresses: [
        new PublicKey(accounts.feeVault),
      ],
      authority: provider.wallet.publicKey,
      lookupTable: new PublicKey(lookupTablePubkey),
      payer: provider.wallet.publicKey,
    });

    await sendTransactionV0([ix]);

    await printAddressLookupTable(lookupTablePubkey);
  });
});*/

describe("create-lut-sol", () => {
  // Configure the client to use the local cluster.
  const accounts = accs.dev.sol;

  const program = anchor.workspace.Pluto as Program<Pluto>;

  let lookupTablePubkey: PublicKey;

  it("Create an Address Lookup Table", async () => {
    let ix: anchor.web3.TransactionInstruction;
    [ix, lookupTablePubkey] = anchor.web3.AddressLookupTableProgram.createLookupTable({
      authority: provider.wallet.publicKey,
      payer: provider.wallet.publicKey,
      recentSlot: await provider.connection.getSlot(),
    });

    await sendTransactionV0([ix]);

    console.log("Pubkeys from generated keypairs:");
    console.log(`LUT Account #1: ${provider.wallet.publicKey}`);
    console.log(`LUT Account Addr #1: ${lookupTablePubkey}`);

    ix = anchor.web3.AddressLookupTableProgram.extendLookupTable({
        addresses: [
          new PublicKey(accounts.protocol),
          new PublicKey(accounts.tokenProgramA),
          new PublicKey(accounts.tokenMintA),
          new PublicKey(accounts.tokenProgramB),
          new PublicKey(accounts.tokenMintB),
          new PublicKey(accounts.tokenCollateralPriceOracle),
          new PublicKey(accounts.nativeCollateralPriceOracle),
          new PublicKey(accounts.earnConfig),
          new PublicKey(accounts.earnStats),
          new PublicKey(accounts.earnVaultAuthority),
          new PublicKey(accounts.earnVault),
          new PublicKey(accounts.earnVaultLiquidity),
          new PublicKey(accounts.leverageConfig),
          new PublicKey(accounts.leverageStats),
          new PublicKey(accounts.leverageVaultAuthority),
          new PublicKey(accounts.leverageVault),
          new PublicKey(accounts.leverageVaultTokenCollateralLiquidity),
          new PublicKey(accounts.leverageVaultNativeCollateralLiquidity),
        ],
        authority: provider.wallet.publicKey,
        lookupTable: new PublicKey(lookupTablePubkey),
        payer: provider.wallet.publicKey,
      });

      await sendTransactionV0([ix]);

      await printAddressLookupTable(lookupTablePubkey);
  });
});

/*describe("extend-lut-sol", () => {
  // Configure the client to use the local cluster.
  const accounts = accs.dev.sol;

  const program = anchor.workspace.Pluto as Program<Pluto>;

  let lookupTablePubkey: PublicKey;

  it("Create an Address Lookup Table", async () => {
    let ix: anchor.web3.TransactionInstruction;
    let lookupTablePubkey = new PublicKey(accounts.lookupTable);

    ix = anchor.web3.AddressLookupTableProgram.extendLookupTable({
      addresses: [
        new PublicKey(accounts.earnStats),
        new PublicKey(accounts.leverageStats),
      ],
      authority: provider.wallet.publicKey,
      lookupTable: new PublicKey(lookupTablePubkey),
      payer: provider.wallet.publicKey,
    });

    await sendTransactionV0([ix]);

    await printAddressLookupTable(lookupTablePubkey);
  });
});*/

export async function sendTransactionV0(
    instructions: anchor.web3.TransactionInstruction[],
): Promise<void> {
  let blockhash = await provider.connection
      .getLatestBlockhash()
      .then((res) => res.blockhash);

  const messageV0 = new anchor.web3.TransactionMessage({
    payerKey: provider.wallet.publicKey,
    recentBlockhash: blockhash,
    instructions,
  }).compileToV0Message();

  const tx = new anchor.web3.VersionedTransaction(messageV0);
  const sx = await provider.sendAndConfirm(tx);

  console.log(`** -- Signature: ${sx}`);
}

export async function printAddressLookupTable(
    lookupTablePubkey: PublicKey,
): Promise<void> {
  setTimeout(async () => {
    const lookupTableAccount = await provider.connection
        .getAddressLookupTable(lookupTablePubkey)
        .then((res) => res.value);
    console.log(`Lookup Table: ${lookupTablePubkey}`);
    for (let i = 0; i < lookupTableAccount.state.addresses.length; i++) {
      const address = lookupTableAccount.state.addresses[i];
      console.log(`   Index: ${i}  Address: ${address.toBase58()}`);
    }
  }, 2000);
}
