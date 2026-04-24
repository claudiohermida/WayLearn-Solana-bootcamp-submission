use anchor_lang::prelude::*;

// Replace with your actual Program ID after running `anchor keys list`
declare_id!("29Re9NUjg9jNX75fxTixaN8zQr8AfaVMwFxZAMoH17Ni");

#[program]
pub mod bowling_shoe_deposit {
    use super::*;

    //////////////////////////// Instruction: Create Deposit /////////////////////////////////////
    pub fn create_deposit(ctx: Context<CreateDeposit>) -> Result<()> {
        let owner_id = ctx.accounts.owner.key();
        msg!("Creating deposit for owner: {}", owner_id);

        let shoes: Vec<Shoe> = Vec::new();

        ctx.accounts.deposit.set_inner(BowlingShoeDeposit {
            owner: owner_id,
            shoes,
        });
        Ok(())
    }

    //////////////////////////// Instruction: Add Shoe /////////////////////////////////////
    pub fn add_shoe(ctx: Context<ManageShoe>, gender: Gender, size: Size) -> Result<()> {
        require!(
            ctx.accounts.deposit.owner == ctx.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let shoes = &mut ctx.accounts.deposit.shoes;

        // Check if the shoe type already exists in the vector
        for shoe in shoes.iter_mut() {
            if shoe.gender == gender && shoe.size == size {
                shoe.quantity += 1;
                shoe.available += 1;
                msg!("Increased quantity. Total: {}, Available: {}", shoe.quantity, shoe.available);
                return Ok(());
            }
        }

        // If it does not exist, add a new entry
        shoes.push(Shoe {
            gender,
            size,
            quantity: 1,
            available: 1,
        });
        
        msg!("Added new shoe type to deposit.");
        Ok(())
    }

    //////////////////////////// Instruction: Fetch Available Shoes /////////////////////////////////////
    pub fn fetch_available_shoes(ctx: Context<ManageShoe>, gender: Gender, size: Size) -> Result<u32> {
        let shoes = &ctx.accounts.deposit.shoes;

        for shoe in shoes.iter() {
            if shoe.gender == gender && shoe.size == size {
                msg!("Available shoes for {:?} {:?}: {}", gender, size, shoe.available);
                return Ok(shoe.available);
            }
        }

        msg!("Shoe type not found in deposit. Available: 0");
        Ok(0)
    }

    //////////////////////////// Instruction: Borrow Shoe /////////////////////////////////////
    pub fn borrow_shoe(ctx: Context<ManageShoe>, gender: Gender, size: Size) -> Result<()> {
        require!(
            ctx.accounts.deposit.owner == ctx.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let shoes = &mut ctx.accounts.deposit.shoes;

        for shoe in shoes.iter_mut() {
            if shoe.gender == gender && shoe.size == size {
                require!(shoe.available > 0, Errores::NoShoeAvailable);
                
                shoe.available -= 1;
                msg!("Shoe borrowed. Remaining available: {}", shoe.available);
                return Ok(());
            }
        }

        Err(Errores::NotExistentShoe.into())
    }

    //////////////////////////// Instruction: Return Shoe /////////////////////////////////////
    pub fn return_shoe(ctx: Context<ManageShoe>, gender: Gender, size: Size) -> Result<()> {
        require!(
            ctx.accounts.deposit.owner == ctx.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let shoes = &mut ctx.accounts.deposit.shoes;

        for shoe in shoes.iter_mut() {
            if shoe.gender == gender && shoe.size == size {
                require!(shoe.available < shoe.quantity, Errores::ShoeNotBorrowed);
                
                shoe.available += 1;
                msg!("Shoe returned. Total available: {}", shoe.available);
                return Ok(());
            }
        }

        Err(Errores::NotExistentShoe.into())
    }

    //////////////////////////// Instruction: Eliminate Shoe /////////////////////////////////////
    pub fn eliminate_shoe(ctx: Context<ManageShoe>, gender: Gender, size: Size) -> Result<()> {
        require!(
            ctx.accounts.deposit.owner == ctx.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let shoes = &mut ctx.accounts.deposit.shoes;

        for i in 0..shoes.len() {
            if shoes[i].gender == gender && shoes[i].size == size {
                // Check if we have at least 1 in quantity and available
                require!(
                    shoes[i].quantity >= 1 && shoes[i].available >= 1, 
                    Errores::NotExistentShoe
                );

                shoes[i].quantity -= 1;
                shoes[i].available -= 1;
                msg!("Shoe eliminated. Total: {}, Available: {}", shoes[i].quantity, shoes[i].available);

                // Optional cleanup: if quantity reaches 0, remove the entry entirely to free up vector space
                if shoes[i].quantity == 0 {
                    shoes.remove(i);
                    msg!("Shoe entry fully removed from deposit.");
                }

                return Ok(());
            }
        }

        Err(Errores::NotExistentShoe.into())
    }
}

////////////////////////////////// Errors //////////////////////////////////

#[error_code]
pub enum Errores {
    #[msg("Error: You are not the owner of this deposit.")]
    NoEresElOwner,
    #[msg("Error: No shoes of this type are currently available.")]
    NoShoeAvailable,
    #[msg("Error: This shoe has not been borrowed (available == quantity).")]
    ShoeNotBorrowed,
    #[msg("Error: Shoe does not exist or insufficient quantity to eliminate.")]
    NotExistentShoe,
}

////////////////////////////////// State & Types //////////////////////////////////

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub enum Gender {
    Male,
    Female,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub enum Size {
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Shoe {
    pub gender: Gender,
    pub size: Size,
    pub quantity: u32,
    pub available: u32,
}

#[account]
#[derive(InitSpace)]
pub struct BowlingShoeDeposit {
    pub owner: Pubkey,
    
    // There are 2 genders * 6 sizes = 12 possible combinations.
    // Setting max_len to 15 gives us plenty of room to store all combinations safely.
    #[max_len(15)] 
    pub shoes: Vec<Shoe>,
}

////////////////////////////////// Contexts //////////////////////////////////

#[derive(Accounts)]
pub struct CreateDeposit<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + BowlingShoeDeposit::INIT_SPACE, 
        seeds = [b"deposit", owner.key().as_ref()], 
        bump
    )]
    pub deposit: Account<'info, BowlingShoeDeposit>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ManageShoe<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub deposit: Account<'info, BowlingShoeDeposit>,
}