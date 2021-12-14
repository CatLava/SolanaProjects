// need to update program ID in this file and toml
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_twitter {
    use super::*;
    // implementing the tweet function
    pub fn send_tweet(ctx: Context<SendTweet>, topic: String, content: String) -> ProgramResult{
        // access the tweet account via ctx
        // info found in the the structs
        let tweet &mut Account<Tweet> = &mut ctx.accounts.tweet;
        let author: &Signer = &ctx.accounts.author;
        // clock for timestamp function
        let clock: Clock = Clock::get().unwrap();
        // checks on content
        if topic.chars().count() > 50 {
            return Err(ErrorCode::TopicTooLong.into())

        }

        if content.chars().count() > 280 {
            return Err(ErrorCode::ContentTooLong.into())
        }

        // final output below
        // get the authors public key via a reference
        tweet.author = *author.key;
        tweet.timestamp = clock.unix_timestamp;
        tweet.topic = topic;
        tweet.content = content;
        Ok(())
    }
}

// adding account means public key is included with the transaction

#[derive(Accounts)]
pub struct SendTweet<'info> {
     // Account of type struct tweet is incoming
    #[account(init, payer = author, space = Tweet::LEN)]
    pub tweet: Account<'info, Tweet>,
    // this ensures signing with private key
    #[account(mut)]
    pub author: Signer<'info>,
    // official system program from Solana
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SendTweet<'info> {
    #[account(init)]
    pub tweet: Account<'info, Tweet>,
    pub author: Signer<'info>,
    pub system_program: AccountInfo<'info>,
}

// define tweet account
// each tweet will occupy its own account
#[account]
pub struct Tweet {
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: String,
    pub content: String.
}

// discriminator is created of 8 bytes 
// this is created for every new Solana account
const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4; // stores size of string
const MAX_TOPIC_LENGTH:  usize = 50 * 4; // 50 chars max
const MAX_CONTENT_LENGTH: usize = 280 * 4; // 280 chars max

impl Tweet {
    const LEN: usize  = DISCRIMINATOR_LENGTH
    + PUBLIC_KEY_LENGTH // author or account
    + TIMESTAMP_LENGTH // timstamp
    + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH //topic
    + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH;
}

#[error]
pub enum ErrorCode {
    #[msg("The provided topic should be 50 characters long maximum.")]
    TopicTooLong,
    #[msg("The provided content should be 280 characters long maximum.")]
    ContentTooLong,
}