use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvoiceStatus {
    Pending,
    Processing,
    Paid,
    Failed,
    Cancelled,
}

impl InvoiceStatus {
    pub fn can_transition_to(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Pending, Self::Processing)
                | (Self::Processing, Self::Paid)
                | (Self::Processing, Self::Failed)
                | (Self::Failed, Self::Processing)
                | (Self::Pending, Self::Cancelled)
                | (Self::Failed, Self::Cancelled)
        )
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Processing => "processing",
            Self::Paid => "paid",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::InvoiceStatus::*;

    #[test]
    fn allows_valid_transitions() {
        assert!(Pending.can_transition_to(Processing));
        assert!(Processing.can_transition_to(Paid));
        assert!(Processing.can_transition_to(Failed));
        assert!(Failed.can_transition_to(Processing));
    }

    #[test]
    fn rejects_invalid_transitions() {
        assert!(!Pending.can_transition_to(Paid));
        assert!(!Paid.can_transition_to(Pending));
        assert!(!Paid.can_transition_to(Processing));
        assert!(!Cancelled.can_transition_to(Processing));
    }
}
