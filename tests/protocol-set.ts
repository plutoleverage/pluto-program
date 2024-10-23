import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
import { Pluto } from "../target/types/pluto";

describe("protocol-set", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Pluto as Program<Pluto>;

  it("Is initialized!", async () => {
    // Add your test here.
    // const tx = await program.methods.createVault().rpc();
    const tx = await program.methods.protocolSet(
        false, false, true, false
    ).rpc({
        skipPreflight:false
    }).catch(e => console.error(e));

    console.log("Your transaction signature", tx);
  });
});
