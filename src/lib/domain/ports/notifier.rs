pub trait Notifier {
    fn can_send_message(&self) -> bool;
    fn is_dnd_enabled(&self) -> Result<bool, String>;
    fn request_permission(&self) -> bool;
    fn send_notification(&self, title: &str, message: &str);
}
