use ibc::{core::ics02_client::events::NewBlock, events::IbcEvent, Height};

use crate::event::monitor::EventBatch;

/// A command for a [`Worker`](crate::worker::Worker).
#[derive(Debug, Clone)]
pub enum WorkerCmd {
    /// A batch of packet events need to be relayed
    IbcEvents { batch: EventBatch<IbcEvent> },

    /// A new block has been committed
    NewBlock { height: Height, new_block: NewBlock },

    /// Trigger a pending packets clear
    ClearPendingPackets,
}
