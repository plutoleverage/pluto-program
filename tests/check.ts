import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { pluto } from "../target/types/pluto";
import { PublicKey } from "@solana/web3.js";

describe("check", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.pluto as Program<pluto>;

  it("Is initialized!", async () => {
    // Add your test here.
    const programId = new PublicKey("8JABYdaQA9jspWE4oFmBNF1LbKS1nyrNGhQHm94iwURi");
    const wallet = new PublicKey("ExhVu5s79DCoheVsrhVVjFnRxuaApX8uhqD4WYG1Kraf");

    program.account.vaultEarn.fetchAndContext("G3cmGtqvSXju914je2Ud88KV6yJ8jfTB7JBo3QJFDcDP").then((account) => {
        console.log(`Vault Account: ${JSON.stringify(account)}`);
        console.log('Unit Supply: ', account.data.unitSupply.toString());
        console.log('Index: ', account.data.index.toString());
        console.log('Deposit Limit: ', account.data.depositLimit.toString());
        console.log('Withdraw Limit: ', account.data.withdrawLimit.toString());
        console.log('Fund Lent: ', account.data.fundTotal.toNumber() + account.data.fundReward.toNumber() - account.data.fundWithdrawn.toNumber());
        console.log('Fund Borrow: ', account.data.fundBorrowed.toNumber() + account.data.fundBorrowInterest.toNumber() - account.data.fundBorrowRepaid.toNumber());
        console.log('Fund Leverage: ', account.data.fundLeverage.toNumber() + account.data.fundLeverageInterest.toNumber() - account.data.fundLeverageRepaid.toNumber());
        console.log('Fund Total: ', account.data.fundTotal.toNumber());
    });

    program.account.lender.fetchAndContext("GohVEmwvfwcuagXP4f4hiiuZ9rtAUR6XHEPpBSzMUqmL").then((account) => {
        console.log(`Lender Account: ${JSON.stringify(account)}`);
        console.log('Units: ', account.data.unit.toString());
        console.log('Index: ', account.data.index.toString());
        console.log('Principal: ', account.data.principal.toString());
    });

    const [PDA, bump] = PublicKey.findProgramAddressSync([Buffer.from("vault", 'utf8'), wallet.toBuffer()], programId);

    console.log(`PDA: ${PDA}`);
    console.log(`Bump: ${bump}`);
  });
});