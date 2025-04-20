use anchor_lang::prelude::*;

declare_id!("4Ufc5Zo2aiysX12sBWHD2rW8mszhs3RqPLBRyaxWac7p");

#[program]
pub mod anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
