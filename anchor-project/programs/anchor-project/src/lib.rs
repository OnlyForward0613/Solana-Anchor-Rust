use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod myepicproject {
    use super::*;

    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result<()> {
        // Get a reference to the account.
        /*
            Note: We do &mut to get a "mutable reference" to base_account. When we do this it actually gives us the power to make changes to base_account. Otherwise, we'd simply be working w/ a "local copy" of base_account.
        */
        let base_account = &mut ctx.accounts.base_account;

        // Initialize total_gifs.
        base_account.total_gifs = 0;
        Ok(())
    }

    // Another function woo!
    pub fn add_gif(ctx: Context<AddGif>) -> Result<()> {
        // Get a reference to the account and increment total_gifs.
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs += 1;
        Ok(())
    }
}

// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    // Just initializing the BaseAccount here
    /*
        1. 'init' creates a new account owned by our current program.
        2. 'payer = user' tells our program who's paying for the account to be created. In this case, it's the user calling the function.
        3. 'space = 9000' will allocate 9000 bytes of space for our account. The more logic we add to our program, the more space we need.
    */
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,

    // "pub user: Signer<'info>" is data passed into the program that proves to the program that the user calling this program actually owns their wallet account.
    #[account(mut)]
    pub user: Signer<'info>,

    // 'system_program: Program' is basically a reference to the SystemProgram, which is the program that basically runs Solana. It is responsible for a lot of stuff, but one of the main things it does is create accounts on Solana. The SystemProgram is a program the creators of Solana deployed that other programs like ours talk to — it has an id of 11111111111111111111111111111111.
    pub system_program: Program<'info, System>,
}

// Specify what data you want in the AddGif Context.
#[derive(Accounts)]
pub struct AddGif<'info> {
    /* 
        Creates a Context named AddGif that has access to a mutable reference to base_account. "#[account(mut)]" basically means we can actually change the total_gifs value stored on BaseAccount.

        Otherwise, it may change data on it within the function but it wouldn't actually change on account. Now, w/ a "mutable" reference if we edit w/ base_account in the function it'll change data on the account itself. 
    */
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
}
