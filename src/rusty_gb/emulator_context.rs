pub struct EmulatorContext {
    pub paused: bool,
    pub running: bool,
    pub ticks: i64,
}

static EMULATOR_CONTEXT: &EmulatorContext = &EmulatorContext {
    paused: false,
    running: false,
    ticks: 0,
};

impl EmulatorContext {
    pub fn get_emulator_context() -> &'static EmulatorContext {
        return EMULATOR_CONTEXT;
    }
}
