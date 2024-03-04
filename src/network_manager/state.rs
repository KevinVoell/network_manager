enum State {
    Unknown,
    Unmanaged,
    Unavailable,
    Disconnected,
    Prepare,
    Config,
    NeedAuth,
    IPConfig,
    IPCheck,
    Secondaries,
    Activated,
    Deactivating,
    Failed,
}

impl State {
    fn from_code(code: u32) -> Option<Self> {
        match code {
            0 => Some(State::Unknown),
            10 => Some(State::Unmanaged),
            20 => Some(State::Unavailable),
            30 => Some(State::Disconnected),
            40 => Some(State::Prepare),
            50 => Some(State::Config),
            60 => Some(State::NeedAuth),
            70 => Some(State::IPConfig),
            80 => Some(State::IPCheck),
            90 => Some(State::Secondaries),
            100 => Some(State::Activated),
            110 => Some(State::Deactivating),
            120 => Some(State::Failed),
            _ => None,
        }
    }
}
