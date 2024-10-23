import * as anchor from "@coral-xyz/anchor";
import {Program, web3} from "@coral-xyz/anchor";
import { Pluto } from "../target/types/pluto";
import {PublicKey} from "@solana/web3.js";

describe("earn-config-create", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  let provider = anchor.getProvider();
  const vaultPda = new anchor.web3.PublicKey("9n6YchExXkpjqA2uTzuC2sMxFpYXteyhwoKr95q2kZzm");
  const multisigPda = new anchor.web3.PublicKey("CEF6KtbzXd7gpimMBUufDevhQv7iXcCXiFZRgCfbSSwZ");

  const program = anchor.workspace.Pluto as Program<Pluto>;

  it("Is initialized!", async () => {
    // Add your test here.
    // const tx = await program.methods.createVault().rpc();
    const tx = await program.methods.earnConfigCreate(
        2000, // Protocol Fee (2%)
        90000, // LTV (90%)
        100, // Deposit Fee (0%)
        new anchor.BN(1), // Min Deposit Limit (0)
        new anchor.BN(1000000), // Max Deposit Limit (1,000,000)
        100, // Withdraw Fee (0%)
        new anchor.BN(0), // Min Withdraw Limit (0)
        new anchor.BN(1000000), // Max Withdraw Limit (1,000,000)
        100, // Borrow Fee (0%)
        new anchor.BN(0), // Min Borrow Limit (0)
        new anchor.BN(100000), // Max Borrow Limit (1,000,000)
        64000 // Floor Cap (64%)
    ).accounts({
      indexer: "ExhVu5s79DCoheVsrhVVjFnRxuaApX8uhqD4WYG1Kraf",
      feeVault: "4n5JRSsB6dm88JQSiyu7y2m5gJ57rj1acXUCqnRMg6X8"
    }).instruction();

    let [configPDA] = PublicKey.findProgramAddressSync([
        Buffer.from("config_earn_e1", 'utf8'),
        vaultPda.toBuffer(),
    ], program.programId);

    let [configAuthPDA] = PublicKey.findProgramAddressSync([
        Buffer.from("config_earn_auth_e1", 'utf8'),
        configPDA.toBuffer(),
    ], program.programId);

    tx.keys = [
        {pubkey: new PublicKey("C5K9wZA9XNQksbDRYaFxB9DJuLaoNEmDLhtKPQvczBvj"), isSigner: false, isWritable: false},
        {pubkey: new PublicKey("6vY3XeiBFgPihZTV2jJTJDervGFdYrhn7pfWEi9tbF1a"), isSigner: false, isWritable: true},
        {pubkey: configAuthPDA, isSigner: false, isWritable: false},
        {pubkey: configPDA, isSigner: false, isWritable: true},
        {pubkey: vaultPda, isSigner: true, isWritable: true},
        {pubkey: web3.SystemProgram.programId, isSigner: false, isWritable: false},
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
