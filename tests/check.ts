import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Pluto } from "../target/types/pluto";
import { PublicKey } from "@solana/web3.js";
// @ts-ignore
import accs from './accounts.json';
import {accounts} from "@sqds/multisig";

describe("check", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  let accounts = accs.dev.usdc;

  const program = anchor.workspace.Pluto as Program<Pluto>;

  it("Is initialized!", async () => {
    // Add your test here.
    const programId = new PublicKey(accounts.programId);

    let [configPDA, bump] = PublicKey.findProgramAddressSync([
      Buffer.from("obligation_stg", 'utf8'),
      new PublicKey(accounts.tokenMintA).toBuffer(),
      new PublicKey(accounts.tokenMintB).toBuffer(),
      new PublicKey(accounts.keeper).toBuffer(),
    ], program.programId);
    console.log(`Config PDA: ${configPDA} Bump: ${bump}`);

    program.account.earnConfig.fetchAndContext(accounts.earnConfig).then((account) => {
      console.log(`Earn Vault Config: ${JSON.stringify(account)}`);
      console.log(`Vault ATA: ${account.data.indexer.toString()}`);
      console.log(`Deposit Fee: ${account.data.depositFee.toString()}`);
      console.log(`Withdraw Fee: ${account.data.withdrawFee.toString()}`);
      console.log(`Borrow Fee: ${account.data.borrowFee.toString()}`);
    });

    program.account.vaultEarn.fetchAndContext(accounts.earnVault).then((account) => {
      console.log(`Earn Vault Account: ${JSON.stringify(account)}`);
      console.log(`Unit Supply: ${account.data.unitSupply}`);
      console.log(`Unit Borrowed: ${account.data.unitBorrowed}`);
      console.log(`Unit Lent: ${account.data.unitLent}`);
      console.log(`Unit Leverage: ${account.data.unitLeverage}`);
      console.log(`Index: ${account.data.index}`);
      console.log(`Index Updated: ${account.data.lastIndexUpdated}`);
    });

    program.account.stats.fetchAndContext(accounts.earnStats).then((account) => {
      console.log(`Earn Stats Account: ${JSON.stringify(account)}`);
      console.log(`Active User: ${account.data.activeUser}`);
    });

    program.account.lender.fetchAndContext("HRXq7fvZUrmRz7kBWzYQVjdyitWgBcwxuJbKeEEDPSQZ").then((account) => {
      console.log(`Lender Account: ${JSON.stringify(account)}`);
      console.log(`Unit: ${account.data.unit}`);
      console.log(`Index: ${account.data.index}`);
    }).catch((err) => console.log(err));

    program.account.vaultLeverage.fetchAndContext(accounts.leverageVault).then((account) => {
      console.log(`Leverage Vault Account: ${JSON.stringify(account)}`);
      console.log(`Unit Supply: ${account.data.unitSupply}`);
      console.log(`Index: ${account.data.index}`);
      console.log(`Borrowing Unit Supply: ${account.data.borrowingUnitSupply}`);
      console.log(`Borrowing Index: ${account.data.borrowingIndex}`);
    });

    program.account.obligation.fetchAndContext("Ee6AhgJB78D8FYtfyGv6Kq1SD9dahCh43drhKUEW234W").then((account) => {
      console.log(`Obligation Account: ${JSON.stringify(account)}`);
      console.log(`Position[0]: ${JSON.stringify(account.data.positions[0])}`);
      console.log(`Position[0] Unit: ${account.data.positions[0].unit}`);
      console.log(`Position[0] Index: ${account.data.positions[0].avgIndex}`);
      console.log(`Position[0] Borrowing Unit: ${account.data.positions[0].borrowingUnit}`);
      console.log(`Position[0] Borrowing Index: ${account.data.positions[0].avgBorrowingIndex}`);
      console.log(`Position[0] Token to Native Ratio: ${account.data.positions[0].tokenToNativeRatio}`);
      console.log(`Position[1]: ${JSON.stringify(account.data.positions[1])}`);
      //console.log(`Vault Account: ${JSON.stringify(account.data.positions[2])}`);
    }).catch((err) => console.log(err));

    //const PDA = PublicKey.createProgramAddressSync([Buffer.from("vault_earn_d4", 'utf8'), mint.toBuffer(), wallet.toBuffer(),Buffer.from([254])], programId);

    //console.log(`PDA: ${PDA}`);
    //console.log(`Bump: ${bump}`);
  });
});