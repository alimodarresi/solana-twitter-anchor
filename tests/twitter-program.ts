import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TwitterProgram } from "../target/types/twitter_program";

describe("twitter-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TwitterProgram as Program<TwitterProgram>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
