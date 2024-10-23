use anchor_lang::{account, InitSpace};
use anchor_lang::prelude::*;
use derivative::Derivative;
use crate::error::{ErrorLeverage};
use crate::state::Position;

#[derive(InitSpace, Derivative, PartialEq)]
#[derivative(Debug)]
#[account(zero_copy(unsafe))]
#[repr(C)]
pub struct Obligation {
    pub is_initialized: bool,
    pub version: u8,
    pub bump: u8,
    #[derivative(Debug = "ignore")]
    pub align0: [u8; 5],
    pub owner: Pubkey,
    pub protocol: Pubkey,
    pub vault: Pubkey,
    pub borrow_vault: Pubkey,
    pub last_updated: i64,
    pub positions: [Position; 3],
    #[derivative(Debug = "ignore")]
    pub padding1: [u64; 64],
}

impl Default for Obligation {
    fn default() -> Self {
        Self {
            is_initialized: false,
            version: 0,
            bump: 0,
            align0: [0;5],
            owner: Pubkey::default(),
            protocol: Pubkey::default(),
            vault: Pubkey::default(),
            borrow_vault: Pubkey::default(),
            last_updated: 0,
            positions: [Position::default(); 3],
            padding1: [0; 64],
        }
    }
}

impl Obligation {
    pub fn init(&mut self, params: InitObligationParams) -> Result<()> {
        let clock = Clock::get()?;
        *self = Self::default();
        self.is_initialized = true;
        self.version = 1;
        self.bump = params.bump;
        self.owner = params.owner;
        self.protocol = params.protocol;
        self.vault = params.vault;
        self.last_updated = clock.unix_timestamp;
        self.positions = [Position::default(); 3];

        Ok(())
    }

    pub fn generate_id(&mut self) -> Result<Pubkey> {
        if let Some(index) = self.positions
            .iter_mut()
            .position(|p| p.id == Pubkey::default() && p.owner == Pubkey::default()) {
            let id = Pubkey::create_with_seed(
                &self.owner,
                &index.to_string(),
                &self.vault,
            ).unwrap();
            Ok(id)
        } else {
            Err(ErrorLeverage::NoPositionSlotAvailable.into())
        }
    }

    pub fn find_or_add_position(&mut self, id: Pubkey, init_function: impl FnOnce(&mut Position) -> Result<()>,) -> Result<&mut Position> {
        if let Some(index) = self.positions
            .iter()
            .position(|p| p.id == id) {
            let position = &mut self.positions[index];
            position.number = index as i8;
            Ok(position)
        } else if let Some(index) = self.positions
            .iter_mut()
            .position(|p| p.id == Pubkey::default() && p.owner == Pubkey::default()) {
            let position = &mut self.positions[index];
            *position = Position::new();

            init_function(position)?;
            position.number = index as i8;

            Ok(position)
        } else {
            Err(ErrorLeverage::NoPositionSlotAvailable.into())
        }
    }

    pub fn close_position(&mut self, id: Pubkey) -> Result<&mut Position> {
        self.update_time()?;
        if let Some(index) = self.positions
            .iter_mut()
            .position(|p| p.id == id) {
            let position = &mut self.positions[index];
            *position = Position::default();

            Ok(position)
        } else {
            Err(ErrorLeverage::NoPositionSlotAvailable.into())
        }
    }

    pub fn find_pending_funded_position(&mut self) -> Result<&mut Position> {
        if let Some(index) = self.positions
            .iter()
            .position(|p| p.state.fund_amount > 0) {
            Ok(&mut self.positions[index])
        } else {
            Err(ErrorLeverage::NoPendingFundedPositionFound.into())
        }
    }

    pub fn find_pending_borrowed_position(&mut self) -> Result<&mut Position> {
        if let Some(index) = self.positions
            .iter()
            .position(|p| p.state.borrow_amount > 0) {
            Ok(&mut self.positions[index])
        } else {
            Err(ErrorLeverage::NoPendingFundedPositionFound.into())
        }
    }

    pub fn find_pending_leveraged_position(&mut self) -> Result<&mut Position> {
        if let Some(index) = self.positions
            .iter()
            .position(|p| p.state.leveraged_amount > 0) {
            Ok(&mut self.positions[index])
        } else {
            Err(ErrorLeverage::NoPendingFundedPositionFound.into())
        }
    }

    pub fn find_releasable_position(&mut self) -> Result<&mut Position> {
        if let Some(index) = self.positions
            .iter()
            .position(|p|
                p.state.fund_amount == 0 && p.state.borrow_amount == 0 &&
                    p.state.leveraged_amount == 0 && p.unit > 0
            ) {
            Ok(&mut self.positions[index])
        } else {
            Err(ErrorLeverage::NoPendingFundedPositionFound.into())
        }
    }

    pub fn update_time(&mut self) -> Result<()> {
        self.last_updated = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

pub struct InitObligationParams {
    pub bump: u8,
    pub owner: Pubkey,
    pub protocol: Pubkey,
    pub vault: Pubkey,
}