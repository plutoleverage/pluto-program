import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solplay } from "../target/types/solplay";
import { PublicKey } from "@solana/web3.js";

describe("deposit", () => {
  const provider = anchor.AnchorProvider.env();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);


  const program = anchor.workspace.Solplay as Program<Solplay>;

  it("Is initialized!", async () => {
    // Add your test here.
    const programId = new PublicKey("8JABYdaQA9jspWE4oFmBNF1LbKS1nyrNGhQHm94iwURi");
    const tokenProgramId = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
    const mintAccount = new PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
    //const [vaultAccount] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("capital_vault")], program.programId);
    const vaultAccount = new PublicKey("G3cmGtqvSXju914je2Ud88KV6yJ8jfTB7JBo3QJFDcDP");

    const tx = await program.methods.deposit(new anchor.BN(5*100000)).accounts({
      vault: vaultAccount,
      tokenMint: mintAccount,
    }).rpc();

    console.log(`TX: ${tx}`);
  });
});