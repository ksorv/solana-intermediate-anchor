import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Calculator } from "../target/types/calculator";
import { Keypair, SystemProgram } from "@solana/web3.js"

describe("calculator", () => {
  const provider = anchor.AnchorProvider.env()
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const program = anchor.workspace.Calculator as Program<Calculator>;

  let counter = Keypair.generate()

  it("Create Counter account!", async () => {
    const tx = await program.methods.createAccount()
        .accounts({
          counterAccount: counter.publicKey,
          authority: provider.wallet.publicKey,
        })
        .signers([counter])
        .rpc()

    console.log("Create account transaction signature", tx)
  });

it("Can increase Counter!", async () => {
    const tx = await program.methods.increment()
        .accounts({
            counterAccount: counter.publicKey,
        })
        .rpc()

    console.log("Increment transaction signature", tx)
});

it("Can decrease Counter!", async () => {
    const tx = await program.methods.decrement()
        .accounts({
            counterAccount: counter.publicKey,
        })
        .rpc()

    console.log("Decrement transaction signature", tx)
});
});
