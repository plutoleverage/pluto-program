import * as anchor from "@coral-xyz/anchor";
import {Program, web3} from "@coral-xyz/anchor";
import { Pluto } from "../target/types/pluto";
import {PublicKey} from "@solana/web3.js";
// @ts-ignore
import accs from "./accounts.json";

describe("leverage-config-change-keeper-sol", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  let provider = anchor.getProvider();
  const accounts = accs.dev.sol;

  const program = anchor.workspace.Pluto as Program<Pluto>;

  it("Is initialized!", async () => {
    // Add your test here.
    // const tx = await program.methods.createVault().rpc();
    const tx = await program.methods.leverageConfigChangeKeeper(
        //new PublicKey(accounts.keeper)
        new PublicKey("EjVrR37mj6YFLpbb7jH8f5Shdpq9CKpgh8VfiEDjjA9T")
    ).accounts({
      config: new PublicKey(accounts.leverageConfig),
    }).rpc({
        skipPreflight:false
    }).catch(e => console.error(e));

    console.log("Your transaction signature", tx);
  });
});
