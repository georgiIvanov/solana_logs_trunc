use anchor_lang::prelude::*;
use emitter::program::Emitter;

declare_id!("DUKnRfntDqsg2jvN5JUvh8otaCAfQe4Q5etkrdm8tE4D");

#[program]
pub mod logs_trunc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.value = 0;
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, cpi_count: u64, row_length: u64, row_count: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.value += 1;

        let cpi_program = ctx.accounts.emitter_program.to_account_info();
        
        for _ in 0..cpi_count {
            let cpi_ctx: CpiContext<'_, '_, '_, '_, emitter::cpi::accounts::LogEvents<'_>> = CpiContext::new(
                cpi_program.clone(), 
                emitter::cpi::accounts::LogEvents {
                    system_program: ctx.accounts.system_program.to_account_info(),
                }
            );
            let _ = emitter::cpi::log_events(cpi_ctx, row_length, row_count);
        }

        msg!("Deposit executed: amount = 1234, fee = 1, etc, etc");
        Ok(())
    }
}

#[account]
pub struct Counter {
    pub value: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8, seeds = [b"counter"], bump)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, seeds = [b"counter"], bump)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
    pub emitter_program: Program<'info, Emitter>,
    pub system_program: Program<'info, System>,
}
