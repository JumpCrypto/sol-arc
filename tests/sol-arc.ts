import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolArc } from "../target/types/sol_arc";

describe("sol-arc", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolArc as Program<SolArc>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
