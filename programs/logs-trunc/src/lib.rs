use anchor_lang::prelude::*;

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

    pub fn deposit(ctx: Context<Deposit>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.value += 1;

        for i in 0..11 {
            // Create a string with repeating digits based on the iteration number
            let digit = i.to_string();
            let large_string = digit.repeat(1_024_000);
            
            // Log the large string
            msg!("{}", large_string);
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
}
