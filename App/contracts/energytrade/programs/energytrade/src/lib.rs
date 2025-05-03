use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Transfer, MintTo};
use anchor_spl::token;
// Use account_info instead of direct token accounts
use anchor_lang::solana_program::account_info::AccountInfo;

declare_id!("Hi9pyrd668XguaqdZnaSbmZfXeRcrEwpZDvjpmLpjv9N");

#[program]
pub mod energytrade {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>, energy_produced: u64, energy_consumed: u64, energy_price_per_kwh: u64) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        
        if user_account.is_initialized {
            return Err(ErrorCode::UserAlreadyRegistered.into());
        }

        user_account.is_initialized = true;
        user_account.wallet_address = *ctx.accounts.user.key;
        user_account.energy_produced = energy_produced;
        user_account.energy_consumed = energy_consumed;
        
        if energy_produced > energy_consumed {
            user_account.surplus = energy_produced - energy_consumed;
            user_account.deficit = 0;
            user_account.energy_price_per_kwh = energy_price_per_kwh;
        } else {
            user_account.surplus = 0;
            user_account.deficit = energy_consumed - energy_produced;
            user_account.energy_price_per_kwh = 0;
        }
        
        user_account.token_balance = 0;

        // Mint initial tokens to the user
        let amount: u64 = 20 * 10u64.pow(9);
        let cpi_accounts = MintTo {
            mint: ctx.accounts.token_mint.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::mint_to(cpi_ctx, amount)?;
        
        // Update token balance
        user_account.token_balance = amount;

        msg!("User account initialized and tokens given.");
        Ok(())
    }

    pub fn set_energy_price(ctx: Context<SetEnergyPrice>, price_per_kwh: u64) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;

        if user_account.surplus <= 0 {
            return Err(ErrorCode::NotASeller.into());
        }

        user_account.energy_price_per_kwh = price_per_kwh;

        msg!("Seller has set the energy price to {} ETK per kWh.", price_per_kwh);
        Ok(())
    }

    pub fn purchase_energy(ctx: Context<PurchaseEnergy>, kwh: u64) -> Result<()> {
        let buyer_account = &mut ctx.accounts.buyer_account;
        let seller_account = &mut ctx.accounts.seller_account;

        // Check seller has enough energy surplus
        if seller_account.surplus < kwh {
            return Err(ErrorCode::InsufficientEnergy.into());
        }

        let total_cost = kwh * seller_account.energy_price_per_kwh;
        
        // Check buyer has enough tokens
        if buyer_account.token_balance < total_cost {
            return Err(ErrorCode::InsufficientTokens.into());
        }

        // Transfer tokens from buyer to seller
        let cpi_accounts = Transfer {
            from: ctx.accounts.buyer_token_account.to_account_info(),
            to: ctx.accounts.seller_token_account.to_account_info(),
            authority: ctx.accounts.buyer.to_account_info(),
        };
        
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::transfer(cpi_ctx, total_cost)?;

        // Update account balances
        buyer_account.token_balance -= total_cost;
        seller_account.token_balance += total_cost;

        buyer_account.energy_consumed += kwh;
        seller_account.surplus -= kwh;

        msg!("Buyer purchased {} kWh of energy from seller for {} ETK.", kwh, total_cost);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(init, payer = user, space = 8 + 1 + 32 + 8 + 8 + 8 + 8 + 8 + 8)]
    pub user_account: Account<'info, UserAccount>,
    
    /// CHECK: Token account of the user 
    #[account(mut)]
    pub user_token_account: AccountInfo<'info>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    
    pub token_program: Program<'info, Token>,
    
    /// CHECK: Token mint
    #[account(mut)]
    pub token_mint: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetEnergyPrice<'info> {
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct PurchaseEnergy<'info> {
    #[account(mut)]
    pub buyer_account: Account<'info, UserAccount>,
    
    #[account(mut)]
    pub seller_account: Account<'info, UserAccount>,
    
    /// CHECK: Token account of the buyer
    #[account(mut)]
    pub buyer_token_account: AccountInfo<'info>,
    
    /// CHECK: Token account of the seller 
    #[account(mut)]
    pub seller_token_account: AccountInfo<'info>,
    
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct UserAccount {
    pub is_initialized: bool,
    pub wallet_address: Pubkey,
    pub energy_produced: u64,
    pub energy_consumed: u64,
    pub surplus: u64,
    pub deficit: u64,
    pub energy_price_per_kwh: u64,
    pub token_balance: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("User is already registered.")]
    UserAlreadyRegistered,
    
    #[msg("User is not a seller.")]
    NotASeller,
    
    #[msg("Insufficient tokens.")]
    InsufficientTokens,
    
    #[msg("Seller does not have enough energy surplus.")]
    InsufficientEnergy,
}