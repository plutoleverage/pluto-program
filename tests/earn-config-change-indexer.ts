import * as anchor from "@coral-xyz/anchor";
import {Program, web3} from "@coral-xyz/anchor";
import { Pluto } from "../target/types/pluto";
import {PublicKey} from "@solana/web3.js";
// @ts-ignore
import accs from "./accounts.json";

describe("earn-config-create", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  let provider = anchor.getProvider();
  const accounts = accs.staging;

  const program = anchor.workspace.Pluto as Program<Pluto>;

  it("Is initialized!", async () => {
    // Add your test here.
    // const tx = await program.methods.createVault().rpc();
    const tx = await program.methods.earnConfigChangeIndexer(
        new PublicKey(accs.staging.indexer)
    ).rpc({
        skipPreflight:false
    }).catch(e => console.error(e));

    console.log("Your transaction signature", tx);
  });
});
