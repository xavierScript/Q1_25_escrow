use anchor_lang::prelude::*;

declare_id!("8y7fVVXx9tKtJ1geEJA4CMUQBQohUqnmuHD9oRa3kDDH");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
