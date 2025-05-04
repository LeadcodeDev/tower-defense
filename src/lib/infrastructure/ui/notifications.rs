use notify_rust::Notification;
use std::process::Command;

use crate::domain::ports::notifier::Notifier;

pub struct NotifierAdapter;

impl NotifierAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Notifier for NotifierAdapter {
    fn can_send_message(&self) -> bool {
        // First check if DND is enabled
        if self.is_dnd_enabled().unwrap_or(false) {
            return false;
        }

        // Then check if notifications are enabled
        self.request_permission()
    }

    fn is_dnd_enabled(&self) -> Result<bool, String> {
        let output = if cfg!(target_os = "macos") {
            Command::new("defaults")
                .arg("read")
                .arg("com.apple.notificationcenterui")
                .arg("doNotDisturb")
                .output()
        } else if cfg!(target_os = "windows") {
            Command::new("powershell")
                .arg("Get-ItemProperty -Path HKCU:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Notifications\\Settings\\")
                .arg("doNotDisturb")
                .output()
        } else {
            return Err("Unsupported OS".to_string());
        };

        if let Ok(output) = output {
            if output.stdout.starts_with(b"1") {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Err("Failed to check DND status".to_string())
        }
    }

    fn request_permission(&self) -> bool {
        let nc_output = if cfg!(target_os = "macos") {
            Command::new("defaults")
                .arg("read")
                .arg("com.apple.notificationcenterui")
                .arg("enabled")
                .output()
        } else if cfg!(target_os = "windows") {
            Command::new("powershell")
                .arg("Get-ItemProperty -Path HKCU:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Notifications\\Settings\\")
                .arg("enabled")
                .output()
        } else {
            return true; // On Linux, assume notifications are enabled by default
        };

        if let Ok(output) = nc_output {
            if output.stdout.starts_with(b"0") {
                return false;
            }
            return true;
        }

        true // If we can't check, assume notifications are enabled
    }

    fn send_notification(&self, title: &str, message: &str) {
        if !self.can_send_message() {
            return;
        }

        Notification::new()
            .summary(title)
            .body(message)
            .show()
            .unwrap();
    }
}
