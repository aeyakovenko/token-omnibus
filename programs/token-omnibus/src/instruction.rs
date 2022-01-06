//! Instruction types

use {
    crate::{
        state::{SHA256},
    },
};

pub enum TokenOmnibusInstructions {
    ///   Initializes a new Token-Omnibus AccountSet.
    ///
    ///   0. `[w]` New AccountSet to create.
    NewAccountSet {
        root: SHA256,
    },

    ///   Transfer the amount into the AccountSet's omnibus account for this Mint.
    ///   Store the proof that destination owner owns the amount in the AccountSet.
    ///
    ///   0. `[w]` AccountSet
    ///   1. `[w]` The source token Account
    ///   2. `[w]` The AccountSet's omnibus token Account.
    ///   2. `[]` Destination owner
    ///   1. `[]` The Token Mint Account
    ///   2. `[]` The Token Programm 
    Deposit {
        ///  proof that value is zero
        ///  Proof must start at SHA256(destination owner, amount)
        proof_zero: [SHA256; 20], 

        /// amount must be delegated by the source token Account
        amount: u64,

        /// PDA(Token Mint Account, pda_bump)
        pda_bump: u8,
    },

    ///   Transfer the amount from the AccountSet's omnibus account for this Mint to
    ///   the destination token account. Remove the proof that the amount is held by 
    ///   by owner from the AccountSet.
    ///
    ///   Path is zero'd in the input AccountSet.
    ///
    ///   0. `[w]` AccountSet
    ///   1. `[w]` The destination token Account.
    ///   2. `[w]` The AccountSet's omnibus token Account.
    ///   2. `[s]` Source owner
    ///   1. `[]` The Token Mint Account
    ///   2. `[]` The Token Programm 
    Withdraw {
        ///  proof that value is in the set
        ///  Proof must start at SHA256(source owner, amount)
        ///  User's token account owner is derived from arg 1
        proof_exists: [SHA256; 20], 
        amount: u64,
        pda_bump: u8,
    },
}
