import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Pluto } from "../target/types/pluto";
import { PublicKey } from "@solana/web3.js";
import {createJupiterApiClient} from "@jup-ag/api";
// @ts-ignore
import accs from './accounts.json';

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

describe("leverage-close-pyusd", () => {
  const program = anchor.workspace.Pluto as Program<Pluto>;
  const accounts = accs.dev.pyusd;
  // Define token mints (you'll need to replace these with actual devnet token addresses)
  const tokenMintA = new PublicKey(accounts.tokenMintA);
  const tokenCollateralPriceOracle = new PublicKey(accounts.tokenCollateralPriceOracle);
  const tokenMintB = new PublicKey(accounts.tokenMintB);
  const nativeCollateralPriceOracle = new PublicKey(accounts.nativeCollateralPriceOracle);

  it("Is initialized!", async () => {
    try {
      const jupiterQuoteApi = createJupiterApiClient()

      // Amount to swap (e.g., 1 USDC)
      const position = 0;
      const amountToSwap = 0.640982 * 1e6 // 0.95 JLP (6 decimals)
      // Compute routes
      const quote = await jupiterQuoteApi.quoteGet({
        inputMint: tokenMintB.toString(),
        outputMint: tokenMintA.toString(),
        amount: Math.floor(amountToSwap),
        maxAccounts: 30,
      })

      const {
        computeBudgetInstructions,
        setupInstructions,
        swapInstruction,
        cleanupInstruction,
        addressLookupTableAddresses,
      } = await jupiterQuoteApi.swapInstructionsPost({
        swapRequest: {
          quoteResponse: quote,
          userPublicKey: provider.wallet.publicKey.toBase58(),
          prioritizationFeeLamports: 'auto',
          dynamicSlippage: {
            maxBps: 30,
          }
        },
      })

      console.log(`Compute Budget Instructions: ${computeBudgetInstructions}`);
      console.log(`Setup Instructions: ${setupInstructions}`);
      console.log(`Swap Instruction: ${swapInstruction}`);
      console.log(`Cleanup Instruction: ${cleanupInstruction}`);
      console.log(`Address Lookup Table Addresses: ${addressLookupTableAddresses}`);

      const vaultAccount = new PublicKey(accounts.leverageVault);

      const close_ix = await program.methods.leverageVaultClose(position).accounts({
        vault: vaultAccount,
      }).instruction();

      const release_ix = await program.methods.leverageVaultRelease(position).accounts({
        vault: vaultAccount,
      }).instruction();

      const repay_borrow_ix = await program.methods.leverageVaultRepayBorrow(position).accounts({
        vault: vaultAccount,
      }).instruction();

      const closing_ix = await program.methods.leverageVaultClosing(position).accounts({
        vault: vaultAccount,
      }).instruction();

      let instructions = [
        anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
          units: 1000000
        }),
        anchor.web3.ComputeBudgetProgram.setComputeUnitPrice({
          microLamports: 0,
        }),
        close_ix,
        release_ix,
        ...setupInstructions.map(instructionDataToTransactionInstruction),
        instructionDataToTransactionInstruction(swapInstruction),
        repay_borrow_ix,
        closing_ix,
        instructionDataToTransactionInstruction(cleanupInstruction),
      ].filter((instruction) => {
        return instruction != null;
      });

      const blockhash = (await provider.connection.getLatestBlockhash()).blockhash;

      // If you want, you can add more lookup table accounts here
      const addressLookupTableAccounts = await getAdressLookupTableAccounts([
        accounts.lookupTable,
        ...addressLookupTableAddresses
      ]);
      const messageV0 = new anchor.web3.TransactionMessage({
        payerKey: provider.wallet.publicKey,
        recentBlockhash: blockhash,
        instructions,
      }).compileToV0Message(addressLookupTableAccounts);
      const trx = new anchor.web3.VersionedTransaction(messageV0);

      /*let simulation = await provider.connection.simulateTransaction(trx);
      console.log(simulation);
      console.log(simulation.value.err);
      simulation.value.logs.forEach(log => console.log(log));*/

      let tx = await provider.sendAndConfirm(trx,[], {skipPreflight: false});
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