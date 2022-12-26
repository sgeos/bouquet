fn on_initialize() {
  return new_mailbox().push_debug_log("Simulation: Initialize");
}

fn on_update(delta_t) {
  let frame = delta_t + 1001;
  let mailbox = new_mailbox();
  mailbox = mailbox.push_debug_log(`Simulation: Update (delta_t = ${delta_t}, frame = ${frame})`);
  return mailbox;
}

fn on_terminate() {
  return new_mailbox().push_debug_log("Simulation: Terminate");
}

