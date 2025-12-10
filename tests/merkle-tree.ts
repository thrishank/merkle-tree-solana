import * as anchor from "@coral-xyz/anchor";
import { MerkleTree as MT } from "merkletreejs";
import { Program } from "@coral-xyz/anchor";
import { MerkleTree } from "../target/types/merkle_tree";
import { PublicKey } from "@solana/web3.js";
import { createHash } from "crypto";

describe("merkle-tree", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.merkleTree as Program<MerkleTree>;

  const address: PublicKey[] = [
    new PublicKey("thrbabBvANwvKdV34GdrFUDXB6YMsksdfmiKj2ZUV3m"),
    new PublicKey("thrbabBvANwvKdV34GdrFUDXB6YMsksdfmiKj2ZUV33"),
    new PublicKey("thrbabBvANwvKdV34GdrFUDXB6YMsksdfmiKj2ZU333"),
    new PublicKey("hhrbabBvANwvKdV34GdrFUDXB6YMsksdfmiKj2ZU333"),
  ];

  function sha256(buf: Buffer): Buffer {
    return createHash("sha256").update(buf).digest();
  }

  const leaves = [];

  for (let i = 0; i < address.length; i++) {
    const hash = sha256(address[i].toBuffer());
    leaves.push(hash);
  }

  const tree = new MT(leaves, sha256, { sortPairs: false });

  const leaf = leaves[3];

  const proofAsNumbers = tree.getProof(leaf).map((p) => Array.from(p.data));

  // it("Is initialized!", async () => {
  //   const tx = await program.methods
  //     .initRoot(tree.getRoot().toJSON().data, new anchor.BN(4))
  //     .accounts({
  //       payer: program.provider.wallet.publicKey,
  //     })
  //     .rpc();
  //   console.log(tx);
  // });

  it("verfiy", async () => {
    const tx = await program.methods
      .verifyInclusion(Array.from(leaf), new anchor.BN(3), proofAsNumbers)
      .accounts({
        merkleAccount: new PublicKey(
          "Cxaa2HHcV4WYbWDmAhwAt4r1Lr8GxyqsTtfPqPu7m2Wu"
        ),
        address: address[3],
      })
      .rpc();
    console.log(tx);
  });
});
