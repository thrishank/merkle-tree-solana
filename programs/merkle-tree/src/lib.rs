use anchor_lang::{prelude::*, solana_program::hash};

declare_id!("DZ166x6NofsTFjc4Q9RyAY77cyiM9jJryFu7gXe65zEA");

#[program]
pub mod merkle_tree {
    use super::*;

    pub fn init_root(ctx: Context<InitRoot>, root: [u8; 32], tree_size: u64) -> Result<()> {
        let merkle = &mut ctx.accounts.merkle_account;
        merkle.root = root;
        merkle.tree_size = tree_size;
        Ok(())
    }

    pub fn verify_inclusion(
        ctx: Context<Verify>,
        leaf: [u8; 32],
        leaf_index: u64,
        proof: Vec<[u8; 32]>,
    ) -> Result<()> {
        let address_hash = hash::hash(&ctx.accounts.address.key.to_bytes()).to_bytes();
        require!(address_hash == leaf, MerkleError::WrongAddress);
        let computed = compute_root_from_proof(leaf, leaf_index, &proof)?;
        let merkle = &ctx.accounts.merkle_account;
        require!(computed == merkle.root, MerkleError::ProofMismatch);
        Ok(())
    }
}

fn compute_root_from_proof(leaf: [u8; 32], mut index: u64, proof: &[[u8; 32]]) -> Result<[u8; 32]> {
    let mut current = leaf;

    for sibling in proof.iter() {
        let mut concat = [0u8; 64];
        if (index & 1) == 0 {
            // current is left, sibling is right
            concat[..32].copy_from_slice(&current);
            concat[32..].copy_from_slice(sibling);
        } else {
            // sibling is left, current is right
            concat[..32].copy_from_slice(sibling);
            concat[32..].copy_from_slice(&current);
        }
        current = hash::hash(&concat).to_bytes(); // sha256
        index >>= 1;
    }

    Ok(current)
}

#[derive(Accounts)]
pub struct InitRoot<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 8,
        seeds = [b"merkle_tree"],
        bump
    )]
    pub merkle_account: Account<'info, MerkleAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Verify<'info> {
    /// CHECK: adress
    pub address: UncheckedAccount<'info>,

    pub merkle_account: Account<'info, MerkleAccount>,
}

#[account]
pub struct MerkleAccount {
    pub root: [u8; 32],
    pub tree_size: u64,
}

#[error_code]
pub enum MerkleError {
    #[msg("Account already initialized")]
    AlreadyInitialized,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Proof does not match stored root")]
    ProofMismatch,
    #[msg("Address doesn't match the intital leaf")]
    WrongAddress,
}
