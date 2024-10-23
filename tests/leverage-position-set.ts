import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Pluto } from "../target/types/pluto";
import { PublicKey } from "@solana/web3.js";
import {createJupiterApiClient} from "@jup-ag/api";
// @ts-ignore
import accs from './accounts.json';

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

describe("leverage-position-set", () => {
  // Configure the client to use the local cluster.

  const program = anchor.workspace.Pluto as Program<Pluto>;
  const accounts = accs.staging;
  // Define token mints (you'll need to replace these with actual devnet token addresses)
  const tokenMintA = new PublicKey(accounts.tokenMintA);
  const tokenCollateralPriceOracle = new PublicKey(accounts.tokenCollateralPriceOracle);
  const tokenMintB = new PublicKey(accounts.tokenMintB);
  const nativeCollateralPriceOracle = new PublicKey(accounts.nativeCollateralPriceOracle);

  it("Is initialized!", async () => {
    try {
      const vaultAccount = new PublicKey(accounts.leverageVault);

      const set_safety_ix = await program.methods.leverageVaultSetSafetyMode(0, true).accounts({
        vault: vaultAccount,
      }).instruction();

      const set_emergency_eject_ix = await program.methods.leverageVaultSetEmergencyEject(0, true).accounts({
        vault: vaultAccount,
      }).instruction();

      const set_profit_taker_ix = await program.methods.leverageVaultSetProfitTaker(0, true, 10000, 10000).accounts({
        vault: vaultAccount,
      }).instruction();

      let instructions = [
        anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
          units: 200000
        }),
        anchor.web3.ComputeBudgetProgram.setComputeUnitPrice({
          microLamports: 0,
        }),
        set_safety_ix,
        set_emergency_eject_ix,
        set_profit_taker_ix
      ].filter((instruction) => {
        return instruction != null;
      });

      const blockhash = (await provider.connection.getLatestBlockhash()).blockhash;

      const messageV0 = new anchor.web3.TransactionMessage({
        payerKey: provider.wallet.publicKey,
        recentBlockhash: blockhash,
        instructions,
      }).compileToV0Message();
      const trx = new anchor.web3.VersionedTransaction(messageV0);

      /*let simulation = await provider.connection.simulateTransaction(trx);
      console.log(simulation);
      console.log(simulation.value.err);
      simulation.value.logs.forEach(log => console.log(log));*/

      let tx = await provider.sendAndConfirm(trx,[], {skipPreflight: true});
      console.log(`TX: ${tx}`);
    } catch (err) {
      console.error(err)
    }
  });
});

export const getAdressLookupTableAccounts = async (
    keys: string[]
): Promise<anchor.web3.AddressLookupTableAccount[]> => {
  const addressLookupTableAccountInfos =
      await provider.connection.getMultipleAccountsInfo(
          keys.map((key) => new PublicKey(key))
      );

  return addressLookupTableAccountInfos.reduce((acc, accountInfo, index) => {
    const addressLookupTableAddress = keys[index];
    if (accountInfo) {
      const addressLookupTableAccount = new anchor.web3.AddressLookupTableAccount({
        key: new PublicKey(addressLookupTableAddress),
        state: anchor.web3.AddressLookupTableAccount.deserialize(accountInfo.data),
      });
      acc.push(addressLookupTableAccount);
    }

    return acc;
  }, new Array<anchor.web3.AddressLookupTableAccount>());
};

export const instructionDataToTransactionInstruction = (
    instructionPayload: any
) => {
  if (instructionPayload == null) {
    return null;
  }

  return new anchor.web3.TransactionInstruction({
    programId: new PublicKey(instructionPayload.programId),
    keys: instructionPayload.accounts.map((key) => ({
      pubkey: new PublicKey(key.pubkey),
      isSigner: key.isSigner,
      isWritable: key.isWritable,
    })),
    data: Buffer.from(instructionPayload.data, "base64"),
  });
};