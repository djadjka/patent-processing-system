pub mod patent;

use crate::services::scylla::CurrentSession;

pub struct State {
    pub session: CurrentSession,
}
