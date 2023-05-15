#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectionMode {
    #[default]
    Online,
    Offline,
}
