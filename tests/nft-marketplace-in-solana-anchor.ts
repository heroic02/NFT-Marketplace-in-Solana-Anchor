import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftMarketplaceInSolanaAnchor } from "../target/types/nft_marketplace_in_solana_anchor";

describe("nft-marketplace-in-solana-anchor", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.NftMarketplaceInSolanaAnchor as Program<NftMarketplaceInSolanaAnchor>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
