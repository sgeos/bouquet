fn on_initialize() {
  return new_mailbox().push_debug_log("Simulation: Initialize");
}

fn on_update(delta_t) {
  let frame = delta_t + 1001;
  return new_mailbox().push_debug_log(`Simulation: Update $ {delta_t}`);
}

fn on_terminate() {
  return new_mailbox().push_debug_log("Simulation: Terminate");
}

