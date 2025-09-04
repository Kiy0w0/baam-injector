// use discord_rpc_client::Client as DiscordRpcClient; // Temporarily disabled
use std::time::{SystemTime, UNIX_EPOCH};

pub struct DiscordRpc {
    // client: Option<DiscordRpcClient>, // Temporarily disabled
    start_time: u64,
}

impl DiscordRpc {
    pub fn new() -> Self {
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            // client: None, // Temporarily disabled
            start_time,
        }
    }

    pub fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Temporarily disabled
        Ok(())
    }

    pub fn update_presence(&mut self, _state: &str, _details: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Temporarily disabled
        Ok(())
    }

    pub fn update_with_process(&mut self, _process_name: Option<&str>) {
        // Temporarily disabled
    }

    pub fn update_injection_status(&mut self, _success: bool, _process_name: &str) {
        // Temporarily disabled
    }

    pub fn disconnect(&mut self) {
        // Temporarily disabled
    }
}

impl Drop for DiscordRpc {
    fn drop(&mut self) {
        self.disconnect();
    }
}
