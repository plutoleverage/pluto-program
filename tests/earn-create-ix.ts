import * as anchor from "@coral-xyz/anchor";
import {Program, web3} from "@coral-xyz/anchor";
import {getAssociatedTokenAddress, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID} from "@solana/spl-token";
import { Pluto } from "../target/types/pluto";
import {PublicKey} from "@solana/web3.js";

describe("earn-create", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  let provider = anchor.getProvider();
  const vaultPda = new anchor.web3.PublicKey("9n6YchExXkpjqA2uTzuC2sMxFpYXteyhwoKr95q2kZzm");
  const multisigPda = new anchor.web3.PublicKey("CEF6KtbzXd7gpimMBUufDevhQv7iXcCXiFZRgCfbSSwZ");
  const tokenMint = new anchor.web3.PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
  let usdcFeed = anchor.utils.bytes.utf8.encode("eaa020c61cc479712813461ce153894a96a6c00b21ed0cfc2798d1f9a9e9c94a");

  const program = anchor.workspace.Pluto as Program<Pluto>;

  it("Is initialized!", async () => {
    // Add your test here.
    // const tx = await program.methods.createVault().rpc();
    const tx = await program.methods.earnVaultCreate(Array.from(usdcFeed)).accounts({
      tokenMint: tokenMint,
      priceOracle: new anchor.web3.PublicKey("Dpw1EAVrSB1ibxiDQyTAW6Zip3J4Btk2x4SgApQCeFbX"),
    }).instruction();

    const configPDA = new PublicKey("3rJ7EouvFRbpwK2agk5PLB42okrRo8d6SEuDJcAburuG");

    let [earnPDA] = PublicKey.findProgramAddressSync([
      Buffer.from("vault_earn_e1", 'utf8'),
      tokenMint.toBuffer(),
      vaultPda.toBuffer(),
    ], program.programId);

    let [earnAuthPDA] = PublicKey.findProgramAddressSync([
      Buffer.from("vault_earn_auth_e1", 'utf8'),
      earnPDA.toBuffer(),
    ], program.programId);

    let earnATA = await getAssociatedTokenAddress(tokenMint, earnAuthPDA, true);

    tx.keys = [
      {pubkey: configPDA, isSigner: false, isWritable: true},
      {pubkey: earnAuthPDA, isSigner: false, isWritable: false},
      {pubkey: earnPDA, isSigner: false, isWritable: true},
      {pubkey: vaultPda, isSigner: true, isWritable: true},
      {pubkey: earnATA, isSigner: false, isWritable: true},
      {pubkey: tokenMint, isSigner: false, isWritable: false},
      {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
      {pubkey: web3.SystemProgram.programId, isSigner: false, isWritable: false},
      {pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
      {pubkey: web3.SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
    ];

    let trx = new web3.TransactionMessage(
        {
          payerKey: vaultPda,
          recentBlockhash: (await provider.connection.getLatestBlockhash()).blockhash,
          instructions: [tx],
        }
    );
    console.log("Your transaction signature", tx.keys);

    let Base58 = require("bs58");
    console.log("Your transaction signature", Base58.encode(trx.compileToLegacyMessage().serialize()));
  });
});
