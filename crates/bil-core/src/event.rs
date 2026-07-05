use crate::primitives::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KernelEvent {
    pub id: EventId,
    pub subject: IdentityId,
    pub authority: AuthorityRef,
    pub time: TimeAnchor,
    pub event_type: EventTypeId,
    pub evidence: EvidenceHash,
    pub value: Option<Commitment>,
    pub state: Option<Commitment>,
    pub previous: Option<Hash32>,
}

impl KernelEvent {
    pub const fn new(
        id: EventId,
        subject: IdentityId,
        authority: AuthorityRef,
        time: TimeAnchor,
        event_type: EventTypeId,
        evidence: EvidenceHash,
        value: Option<Commitment>,
        state: Option<Commitment>,
        previous: Option<Hash32>,
    ) -> Self {
        Self {
            id,
            subject,
            authority,
            time,
            event_type,
            evidence,
            value,
            state,
            previous,
        }
    }
}
