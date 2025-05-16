use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod citizenship_verifier {
    use super::*;

    pub fn store_proof(ctx: Context<StoreProof>, proof_data: ProofData) -> Result<()> {
        // Validate string lengths
        require!(proof_data.user_id.len() <= 50, ErrorCode::StringTooLong);
        require!(proof_data.citizenship_number.len() <= 20, ErrorCode::StringTooLong);
        require!(proof_data.name.len() <= 100, ErrorCode::StringTooLong);
        require!(proof_data.dob.len() <= 10, ErrorCode::StringTooLong);
        
        let proof_account = &mut ctx.accounts.proof_account;
        proof_account.user_id = proof_data.user_id;
        proof_account.citizenship_number = proof_data.citizenship_number;
        proof_account.name = proof_data.name;
        proof_account.dob = proof_data.dob;
        proof_account.zk_proof = proof_data.zk_proof;
        proof_account.bump = ctx.bumps.proof_account; // Store the bump
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(proof_data: ProofData)]
pub struct StoreProof<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 4 + 50 + 4 + 20 + 4 + 100 + 4 + 10 + 4 + 1024 + 1, // Discriminator + fields + bump
        seeds = [b"proof", user.key().as_ref(), proof_data.user_id.as_bytes()],
        bump
    )]
    pub proof_account: Account<'info, ProofAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ProofAccount {
    pub user_id: String,            // 4 + 50
    pub citizenship_number: String, // 4 + 20
    pub name: String,               // 4 + 100
    pub dob: String,                // 4 + 10
    pub zk_proof: Vec<u8>,          // 4 + 1024
    pub bump: u8,                   // 1
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ProofData {
    pub user_id: String,
    pub citizenship_number: String,
    pub name: String,
    pub dob: String,
    pub zk_proof: Vec<u8>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("String exceeds maximum allowed length")]
    StringTooLong,
}