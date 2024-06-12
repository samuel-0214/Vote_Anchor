use anchor_lang::prelude::*;

declare_id!("3xRJ5rqBVnernjGyvFxXLmi9sLgVsfStCridStg98fso");

#[program]
pub mod vote_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _url: String) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?; //initialize acc with bumps
        Ok(())
    }

    pub fn upvote(ctx: Context<Vote>, _url: String) -> Result<()> {
        ctx.accounts.upvote()?;
        Ok(())
    }

    pub fn downvote(ctx: Context<Vote>, _url: String) -> Result<()> {
        ctx.accounts.downvote()?;
        Ok(())
    }
}

//   Account specs for contexts

// 1. Initialize Contexts
#[derive(Accounts)]
#[instruction(_url: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>, //who pays for the account creation
    #[account(
        init, //init constraint because the account doesn't exist, has to be init
        payer = payer,
        seeds = [_url.as_bytes().as_ref()], //url is passed from instruction
        bump, //find canonical bump
        space = VoteState::INIT_SPACE
    )]
    pub vote_state: Account<'info, VoteState>, // the vote state PDA account
    pub system_program: Program<'info, System>,
}

// Implementing functionality
impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        //all bumps found are stored in InitializeBumps (__ContextName__ followed by __Bumps__ )
        self.vote_state.score = 0;
        self.vote_state.bump = bumps.vote_state;
        Ok(())
    }
}

// 2. Vote Context

#[derive(Accounts)]
#[instruction(_url: String)]
pub struct Vote<'info> {
    #[account(mut)]
    pub payer: Signer<'info>, //who pays for the account creation
    #[account(
         mut,
        seeds=[_url.as_bytes().as_ref()],     //url is passed from instruction
        bump = vote_state.bump,//fetch  bump
     )]
    pub vote_state: Account<'info, VoteState>, // the vote state PDA account
}

// Implementing functionality
impl<'info> Vote<'info> {
    pub fn upvote(&mut self) -> Result<()> {
        //&mut self : reference to struct Vote itself
        self.vote_state.score += 1;
        Ok(())
    }

    pub fn downvote(&mut self) -> Result<()> {
        self.vote_state.score -= 1;
        Ok(())
    }
}

// All custom accounts - PDAs
#[account]
pub struct VoteState {
    pub score: i64,
    pub bump: u8,
}

impl Space for VoteState {
    const INIT_SPACE: usize = 8 + 8 + 1;
}
