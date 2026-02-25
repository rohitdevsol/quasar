use crate::prelude::*;

define_account!(pub struct Clock => [checks::Address]);

impl crate::traits::Program for Clock {
    const ID: Address = Address::new_from_array([
        6, 167, 213, 23, 24, 199, 116, 201, 40, 86, 99, 152, 105, 29, 94, 182, 139, 94, 184, 163,
        155, 75, 109, 92, 115, 85, 91, 33, 0, 0, 0, 0,
    ]);
}

impl Clock {
    #[inline(always)]
    pub fn get(
        &self,
    ) -> Result<solana_account_view::Ref<'_, crate::sysvars::clock::Clock>, ProgramError> {
        crate::sysvars::clock::Clock::from_account_view(self.to_account_view())
    }

    /// Access clock data without borrow tracking or address verification.
    ///
    /// # Safety
    ///
    /// The caller must ensure this Clock account was already validated via
    /// `from_account_view` (which checks the address). Account data must
    /// not be mutably borrowed.
    #[inline(always)]
    pub unsafe fn get_unchecked(&self) -> &crate::sysvars::clock::Clock {
        crate::sysvars::clock::Clock::from_bytes_unchecked(self.to_account_view().borrow_unchecked())
    }
}
