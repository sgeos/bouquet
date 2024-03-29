use bouquet_core::rendering::BouquetInteger;

#[derive(Debug, Default, Clone)]
pub struct ProgramState {
    pub persistent_data: PersistentData,
    pub last_frame_data: FrameData,
    pub next_frame_data: FrameData,
}

impl ProgramState {
    pub fn new() -> ProgramState {
        ProgramState {
            persistent_data: PersistentData::new(),
            last_frame_data: FrameData::new(),
            next_frame_data: FrameData::new(),
        }
    }

    pub fn next_frame(&mut self) {
        self.last_frame_data = self.next_frame_data.clone();
    }
}

#[derive(Debug, Default, Clone)]
pub struct PersistentData {
    pub done: bool,
}

impl PersistentData {
    pub fn new() -> PersistentData {
        PersistentData { done: false }
    }
}

#[derive(Debug, Default, Clone)]
pub struct FrameData {
    pub frame: BouquetInteger,
}

impl FrameData {
    pub fn new() -> FrameData {
        FrameData { frame: 0 }
    }
}
