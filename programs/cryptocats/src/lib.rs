use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("11111111111111111111111111111111");

#[program]
pub mod cryptocats {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        authority: Pubkey,
        cats_mint: Pubkey,
        threshold: u64,
        claim_mode: u8,
        event_duration: u64,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.authority = authority;
        config.cats_mint = cats_mint;
        config.threshold = threshold;
        config.claim_mode = claim_mode;
        config.event_duration = event_duration;
        config.next_event_id = 1;
        config.next_seed = 1337;
        config.bump = ctx.bumps.config;
        Ok(())
    }

    pub fn create_claim_event(
        ctx: Context<CreateClaimEvent>,
        event_id: u64,
        mode: u8,
        threshold: u64,
        start_slot: u64,
        end_slot: u64,
        max_claims: u64,
    ) -> Result<()> {
        let event = &mut ctx.accounts.event;
        event.config = ctx.accounts.config.key();
        event.authority = ctx.accounts.authority.key();
        event.event_id = event_id;
        event.mode = mode;
        event.threshold = threshold;
        event.start_slot = start_slot;
        event.end_slot = end_slot;
        event.max_claims = max_claims;
        event.claims_minted = 0;
        event.bump = ctx.bumps.event;
        event.seed = ctx.accounts.config.next_seed;
        Ok(())
    }

    pub fn claim_nft(
        ctx: Context<ClaimNft>,
        event_id: u64,
        nonce: u64,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        let event = &mut ctx.accounts.event;
        let claim = &mut ctx.accounts.claim;

        require!(event_id == event.event_id, CryptoCatsError::EventMismatch);
        require!(event.claims_minted < event.max_claims, CryptoCatsError::MaxClaimsReached);
        require!(!claim.claimed, CryptoCatsError::AlreadyClaimed);
        require!(ctx.accounts.user_cats_ata.amount >= event.threshold, CryptoCatsError::InsufficientBalance);

        let current_slot = Clock::get()?.slot;
        match event.mode {
            0 => require!(ctx.accounts.user_cats_ata.amount >= event.threshold, CryptoCatsError::InsufficientBalance),
            1 => {
                require!(current_slot >= event.start_slot, CryptoCatsError::EventNotStarted);
                require!(current_slot <= event.end_slot, CryptoCatsError::EventExpired);
            }
            2 => {}
            _ => return Err(CryptoCatsError::UnsupportedMode.into()),
        }

        let traits = derive_traits(config.next_seed.wrapping_add(event_id).wrapping_add(nonce));
        let transfer_cpi_accounts = Transfer {
            from: ctx.accounts.user_cats_ata.to_account_info(),
            to: ctx.accounts.authority_ata.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let transfer_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_cpi_accounts);
        token::transfer(transfer_ctx, 0)?;

        let mint_to_accounts = token::MintTo {
            mint: ctx.accounts.nft_mint.to_account_info(),
            to: ctx.accounts.user_nft_ata.to_account_info(),
            authority: ctx.accounts.program_authority.to_account_info(),
        };
        let mint_to_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            mint_to_accounts,
            &[&[b"program-authority", &[ctx.bumps.program_authority]]],
        );
        token::mint_to(mint_to_ctx, 1)?;

        claim.claimed = true;
        claim.config = config.key();
        claim.event = event.key();
        claim.claimant = ctx.accounts.claimant.key();
        claim.nft_mint = ctx.accounts.nft_mint.key();
        claim.claimed_at_slot = current_slot;
        claim.traits = traits;
        claim.bump = ctx.bumps.claim;

        config.next_seed = config.next_seed.wrapping_add(1);
        event.claims_minted = event.claims_minted.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(authority: Pubkey, cats_mint: Pubkey, threshold: u64, claim_mode: u8, event_duration: u64)]
pub struct InitializeConfig<'info> {
    #[account(init, payer = authority, space = 8 + Config::LEN, seeds = [b"config"], bump)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(event_id: u64, mode: u8, threshold: u64, start_slot: u64, end_slot: u64, max_claims: u64)]
pub struct CreateClaimEvent<'info> {
    #[account(mut)]
    pub config: Account<'info, Config>,
    #[account(init, payer = authority, space = 8 + ClaimEvent::LEN, seeds = [b"event", config.key().as_ref(), &event_id.to_le_bytes()], bump)]
    pub event: Account<'info, ClaimEvent>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(event_id: u64, nonce: u64)]
pub struct ClaimNft<'info> {
    #[account(mut)]
    pub config: Account<'info, Config>,
    #[account(mut, seeds = [b"event", config.key().as_ref(), &event_id.to_le_bytes()], bump = event.bump)]
    pub event: Account<'info, ClaimEvent>,
    #[account(init_if_needed, payer = claimant, space = 8 + ClaimReceipt::LEN, seeds = [b"claim", config.key().as_ref(), event.key().as_ref(), claimant.key().as_ref()], bump)]
    pub claim: Account<'info, ClaimReceipt>,
    #[account(mut)]
    pub claimant: Signer<'info>,
    #[account(mut)]
    pub user_cats_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub nft_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_nft_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: PDA authority for minting
    #[account(seeds = [b"program-authority"], bump)]
    pub program_authority: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct Config {
    pub authority: Pubkey,
    pub cats_mint: Pubkey,
    pub threshold: u64,
    pub claim_mode: u8,
    pub event_duration: u64,
    pub next_event_id: u64,
    pub next_seed: u64,
    pub bump: u8,
}

impl Config {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 1 + 8 + 8 + 8 + 1;
}

#[account]
pub struct ClaimEvent {
    pub config: Pubkey,
    pub authority: Pubkey,
    pub event_id: u64,
    pub mode: u8,
    pub threshold: u64,
    pub start_slot: u64,
    pub end_slot: u64,
    pub max_claims: u64,
    pub claims_minted: u64,
    pub seed: u64,
    pub bump: u8,
}

impl ClaimEvent {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 1 + 8 + 8 + 8 + 8 + 8 + 8 + 1;
}

#[account]
pub struct ClaimReceipt {
    pub config: Pubkey,
    pub event: Pubkey,
    pub claimant: Pubkey,
    pub nft_mint: Pubkey,
    pub claimed_at_slot: u64,
    pub claimed: bool,
    pub traits: [u8; 5],
    pub bump: u8,
}

impl ClaimReceipt {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 32 + 8 + 1 + 5 + 1;
}

fn derive_traits(seed: u64) -> [u8; 5] {
    [
        (seed % 6) as u8,
        ((seed >> 8) % 5) as u8,
        ((seed >> 16) % 4) as u8,
        ((seed >> 24) % 4) as u8,
        ((seed >> 32) % 4) as u8,
    ]
}

#[error_code]
pub enum CryptoCatsError {
    #[msg("The claimant already redeemed this event")]
    AlreadyClaimed,
    #[msg("The wallet does not meet the required balance")]
    InsufficientBalance,
    #[msg("The event does not exist or does not match the given id")]
    EventMismatch,
    #[msg("The event has not started yet")]
    EventNotStarted,
    #[msg("The event has expired")]
    EventExpired,
    #[msg("The event has reached its max claim count")]
    MaxClaimsReached,
    #[msg("The selected claim mode is not supported")]
    UnsupportedMode,
}
