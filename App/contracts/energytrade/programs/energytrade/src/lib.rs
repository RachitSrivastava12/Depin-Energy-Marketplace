use anchor_lang::prelude::*;

declare_id!("Hi9pyrd668XguaqdZnaSbmZfXeRcrEwpZDvjpmLpjv9N");

#[program]
pub mod energytrade {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
