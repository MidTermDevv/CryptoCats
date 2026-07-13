import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CryptoCats } from "../target/idl/cryptocats";

describe("cryptocats", () => {
  it("builds and exposes a claim flow scaffold", async () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.Cryptocats as Program<CryptoCats>;
    expect(program.programId.toBase58()).toBeDefined();
  });
});
