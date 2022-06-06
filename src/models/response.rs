// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

//

// ----------------------------------------------------------------
// Enum
// ----------------------------------------------------------------

/// # ResponseState #
/// Enum for response types
///
/// - `PENDING` - response is pending
/// - `CANCEL` - use to indicate that META + D was entered
/// - `QUIT` - use to indicate that META + C was entered
/// - `COMPLETED` - finished reading response and above combinations not pressed.
#[derive(Debug)]
pub enum ResponseState {
    PENDING,
    CANCEL,
    QUIT,
    COMPLETED,
}

// ----------------------------------------------------------------
// Implementation
// ----------------------------------------------------------------

impl ResponseState {
    pub fn is_cancel(self: &Self) -> bool {
        matches!(*self, ResponseState::CANCEL)
    }

    pub fn is_quit(self: &Self) -> bool {
        matches!(*self, ResponseState::QUIT)
    }

    pub fn is_completed(self: &Self) -> bool {
        matches!(*self, ResponseState::COMPLETED)
    }

    pub fn is_cancel_or_quit(self: &Self) -> bool {
        matches!(*self, ResponseState::CANCEL | ResponseState::QUIT)
    }
}
