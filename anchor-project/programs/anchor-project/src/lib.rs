use anchor_lang::prelude::*;

declare_id!("BcsgHu3sCQkC5DfMjbqM74zJEdtY4m3ZfDqMVjGYeWcU");

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
    // The function now accepts a gif_link param from the user. We also reference the user from the Context
    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        // Build the struct.
        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
        };

        // Add it to the gif_list vector.
        base_account.gif_list.push(item);
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

    // Add the signer who calls AddGif method to the struct so that we can save it
    #[account(mut)]
    pub user: Signer<'info>,
}

/*
   Basically the following line tells Anchor how to serialize/deserialize the struct. Remember, data is being stored in an "account". That account is a file and we serialize our data into binary format before storing it. Then, when we want to retrieve it we'll actually deserialize it. This line takes care of that to make sure our data is properly serialized/deserialized since we're creating a custom struct here.
*/
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    // Create a custom struct for us to work with.
    pub gif_link: String,
    pub user_address: Pubkey, //Pubkey here is a data type
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,

    // Attach a Vector of type ItemStruct to the account
    // Basically you can think of a Vector like an Array
    pub gif_list: Vec<ItemStruct>,
}
