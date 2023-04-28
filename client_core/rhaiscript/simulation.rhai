fn on_initialize() {
  let frame = 0;
  let time = 0;
  return new_mailbox().push_debug_log("Simulation: Initialize");
}

fn on_update(delta_t) {
  let frame = 1;
  let time = delta_t;
  let mailbox = new_mailbox();
  mailbox = mailbox.push_debug_log(`Simulation: Update (delta_t = ${delta_t}, frame = ${frame})`);
  return mailbox;
}

fn on_terminate() {
  return new_mailbox().push_debug_log("Simulation: Terminate");
}

