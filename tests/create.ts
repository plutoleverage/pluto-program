import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solplay } from "../target/types/solplay";

describe("solplay", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Solplay as Program<Solplay>;

  it("Is initialized!", async () => {
    // Add your test here.
    // const tx = await program.methods.createVault().rpc();
    const tx = await program.methods.create(
        8000,
        new anchor.BN("1000000000000000000"),
        new anchor.BN("1000000000000000000"),
    ).accounts({
      tokenMint: new anchor.web3.PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU"),
    }).rpc();
    console.log("Your transaction signature", tx);
  });
});
