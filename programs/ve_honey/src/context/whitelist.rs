use crate::constants::*;
use crate::error::*;
use crate::state::*;
use anchor_lang::prelude::*;
use vipers::*;

#[derive(Accounts)]
pub struct ApproveProgramLockPrivilege<'info> {
    /// Payer of initialization.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// [Locker].
    #[account(
        constraint = locker.admin == locker_admin.key() @ ProtocolError::InvalidLockerAdmin
    )]
    pub locker: Box<Account<'info, Locker>>,
    /// Admin of the [Locker].
    pub locker_admin: Signer<'info>,
    /// [WhitelistEntry].
    #[account(
        init,
        seeds = [
            WHITELIST_ENTRY_SEED.as_bytes(),
            locker.key().as_ref(),
            executable_id.key().as_ref(),
            whitelisted_owner.key().as_ref()
        ],
        bump,
        payer = payer,
        space = std::mem::size_of::<WhitelistEntry>() + 8
    )]
    pub whitelist_entry: Box<Account<'info, WhitelistEntry>>,

    /// CHECK: ProgramId of the program to whitelist.
    pub executable_id: UncheckedAccount<'info>,

    /// CHECK: Owner whitelisted. If set to [System], then the program is whitelisted for all accounts.
    pub whitelisted_owner: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> ApproveProgramLockPrivilege<'info> {
    pub fn process(&mut self, bump: u8) -> Result<()> {
        let whitelist_entry = &mut self.whitelist_entry;

        whitelist_entry.bump = bump;
        whitelist_entry.locker = self.locker.key();
        whitelist_entry.program_id = self.executable_id.key();
        whitelist_entry.owner = self.whitelisted_owner.key();

        emit!(ApproveLockPrivilegeEvent {
            locker: whitelist_entry.locker,
            program_id: whitelist_entry.program_id,
            owner: whitelist_entry.owner,
            timestamp: Clock::get()?.unix_timestamp
        });

        Ok(())
    }
}

impl<'info> Validate<'info> for ApproveProgramLockPrivilege<'info> {
    fn validate(&self) -> Result<()> {
        invariant!(
            self.executable_id.executable,
            "program_id must be an executable"
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct RevokeProgramLockPrivilege<'info> {
    /// Payer.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// [Locker].
    #[account(
        constraint = locker.admin == locker_admin.key() @ ProtocolError::InvalidLockerAdmin
    )]
    pub locker: Box<Account<'info, Locker>>,
    /// Admin of the [Locker].
    pub locker_admin: Signer<'info>,
    /// [WhitelistEntry].
    #[account(mut, close = payer)]
    pub whitelist_entry: Box<Account<'info, WhitelistEntry>>,

    /// CHECK: ProgramId of the program to whitelist.
    pub executable_id: UncheckedAccount<'info>,
}

impl<'info> RevokeProgramLockPrivilege<'info> {
    pub fn process(&self) -> Result<()> {
        emit!(RevokeLockPrivilegeEvent {
            locker: self.whitelist_entry.locker,
            program_id: self.whitelist_entry.program_id,
            timestamp: Clock::get()?.unix_timestamp
        });

        Ok(())
    }
}

impl<'info> Validate<'info> for RevokeProgramLockPrivilege<'info> {
    fn validate(&self) -> Result<()> {
        assert_keys_eq!(self.whitelist_entry.program_id, self.executable_id);

        Ok(())
    }
}

#[event]
/// Event called in [ve_honey::approve_program_lock_privilege].
pub struct ApproveLockPrivilegeEvent {
    /// [Locker].
    #[index]
    pub locker: Pubkey,
    /// ProgramId approved to make CPI calls to [ve_honey::lock].
    pub program_id: Pubkey,
    /// Owner of the [Escrow].
    pub owner: Pubkey,
    /// Timestamp
    pub timestamp: i64,
}

#[event]
pub struct RevokeLockPrivilegeEvent {
    /// [Locker].
    pub locker: Pubkey,
    /// ProgramId approved to make CPI calls to [ve_honey::lock].
    pub program_id: Pubkey,
    /// Timestamp.
    pub timestamp: i64,
}
