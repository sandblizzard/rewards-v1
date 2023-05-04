use bounty_sdk::utils::SBError;

/// status_manager is responsible for posting status
/// to domain

pub struct StatusManager {
    status: String,
}

impl StatusManager {
    pub fn new(status: String) -> StatusManager {
        StatusManager { status }
    }

    pub fn already_posted(&self) -> Result<bool, SBError> {
        Ok(true)
    }

    pub fn post_status<F>(&self, emitter: F) -> Result<(), SBError>
    where
        F: Fn(&str),
    {
        emitter(&self.status);
        Ok(())
    }
}
