// src/config.js
export const NOSTR_RELAYS = [
  'ws://localhost:7777',  // Local relay for testing
  // Add production relays when ready
  // 'wss://relay.damus.io',
  // 'wss://relay.nostr.info',
];

export const COMMAND_TYPES = {
  STATUS: "Status",
  ADD_ITEM: "AddItem",
  REMOVE_ITEM: "RemoveItem",
  CHANGE_PRICE: "ChangePrice",
  REBOOT: "Reboot",
  SHUTDOWN: "Shutdown",
  REQUEST_ADMIN: "RequestAdminState",
  END_ADMIN: "End"
};

export const SIMULATION_MODE = false; // Set to true to enable simulation mode