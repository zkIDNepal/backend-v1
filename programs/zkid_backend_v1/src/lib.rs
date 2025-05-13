use anchor_lang::prelude::*;

declare_id!("3e3z5XtGTcApoi3dVZ25XCzdoYnUAPbkMQ6kHZPa6oCJ");

#[program]
pub mod zkid_backend_v1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
