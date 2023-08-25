#![no_std]
#![feature(rustc_private)]

#[macro_use]
extern crate alloc;
extern crate libc;

mod fermium_renderer;
mod terminal;

use {
    alloc::{boxed::Box, string::ToString, sync::Arc},
    bouquet_client_core::{program_state::ProgramState, simulation::Simulation},
    bouquet_core::{
        message::Message,
        message_bus::{MessageBus, MessageSendee},
        rendering::{BouquetFloat, Color, Renderer},
    },
    core::convert::TryInto,
    fermium::prelude::*,
    fermium_renderer::FermiumRenderer,
    terminal::Terminal,
};

#[no_mangle]
pub extern "C" fn run() {
    let renderer: FermiumRenderer = FermiumRenderer::new("Bouquet Demo", 640, 480).unwrap();
    renderer.clear(Color {
        r: 1.0,
        g: 0.0,
        b: 0.8,
        a: 1.0,
    });
    renderer.present();

    let mut ps = ProgramState::new();
    let mut mb = MessageBus::<ProgramState, Message>::new();
    let simulation = Box::new(Simulation::new());
    let terminal = Box::new(Terminal::new());
    let mut time = unsafe { libc::time(core::ptr::null_mut()) };

    mb.register("simulation", simulation);
    mb.register("terminal", terminal);
    mb.send(
        Arc::new(Message::DebugLog("Hello, Bouquet!".to_string())),
        &mut ps,
    );
    mb.send(Arc::new(Message::Initialize), &mut ps);
    while !ps.persistent_data.done {
        let mut event = SDL_Event::default();
        let pending_events = 0 < unsafe { SDL_PollEvent(&mut event) };
        if pending_events {
            let event_type = unsafe { event.type_ };
            if event_type == SDL_QUIT {
                ps.persistent_data.done = true
            }
        }
        let old_time = time;
        time = unsafe { libc::time(core::ptr::null_mut()) };
        let delta_t = (time - old_time).try_into().unwrap_or(0);
        mb.send(Arc::new(Message::Update(delta_t as BouquetFloat)), &mut ps);
        ps.next_frame();
    }
    mb.unregister("simulation");
    mb.unregister("terminal");
}
