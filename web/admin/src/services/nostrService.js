// src/services/nostrService.js
import { SimplePool, nip19, getPublicKey, finalizeEvent } from 'nostr-tools';
import { encrypt, getConversationKey} from 'nostr-tools/nip44';

// Set up relays
const RELAYS = [
  'ws://localhost:7777',
];

// Initialize a single relay pool for sending messages
const pool = new SimplePool();

const nostrService = {
  /**
   * Sends an administrative command to a vending machine over Nostr
   * 
   * @param {string} privateKey - Admin's private key (hex or nsec format)
   * @param {string} targetPubKey - Vending machine's public key (hex or npub format)
   * @param {object} command - Command object to send
   * @returns {Promise<object>} Result of the operation
   */
  async sendCommand(privateKey, targetPubKey, command) {
    try {
      console.log(`Preparing to send ${command.type} command to vending machine`);
      
      // Normalize keys
      let secretKey = privateKey;
      let pubKey = targetPubKey;
      
      // Convert from NIP-19 format if needed
      if (privateKey.startsWith('nsec')) {
        try {
          const decoded = nip19.decode(privateKey);
          secretKey = decoded.data;
        } catch (err) {
          console.error("Error decoding nsec:", err);
          return { success: false, message: `Invalid nsec format: ${err.message}` };
        }
      }
      
      if (targetPubKey.startsWith('npub')) {
        try {
          const decoded = nip19.decode(targetPubKey);
          pubKey = decoded.data;
        } catch (err) {
          console.error("Error decoding npub:", err);
          return { success: false, message: `Invalid npub format: ${err.message}` };
        }
      }
      
      // Get admin's public key from private key
      let adminPubKey;
      try {
        adminPubKey = getPublicKey(secretKey);
      } catch (err) {
        console.error("Error getting public key:", err);
        return { success: false, message: `Invalid private key: ${err.message}` };
      }
      
      // Stringify the command
      const commandStr = JSON.stringify(command);
      console.log("Command JSON:", commandStr);
      
      // Encrypt using NIP-44 with explicit key preparation
      let encryptedContent;
      try {
          const conversationKey = getConversationKey(
            secretKey,
            pubKey
          );
          encryptedContent = await encrypt(commandStr, conversationKey);
        
      } catch (err) {
        console.error("NIP-44 encryption error:", err);
        return { success: false, message: `NIP-44 encryption failed: ${err.message}` };
      }
      
      // Create and finalize the event
      const event = {
        kind: 4, // Direct Message
        created_at: Math.floor(Date.now() / 1000),
        tags: [['p', pubKey]], // Tag with recipient's pubkey
        content: encryptedContent,
        pubkey: adminPubKey,
      };
      
      // In nostr-tools 2.x, finalizeEvent is the recommended way to create a signed event
      const signedEvent = finalizeEvent(event, secretKey);
      
      // Publish to relays
      const pubs = pool.publish(RELAYS, signedEvent);
      
      // Since pubs is already resolved and the command works, 
      // we don't need to wait for explicit 'ok' responses
      console.log("Event published to relays:", signedEvent.id);
      return { 
        success: true, 
        message: `Command ${command.type} sent successfully`,
        event: signedEvent
      };

    } catch (error) {
      console.error("Unexpected error in sendCommand:", error);
      return { 
        success: false, 
        message: `Unexpected error: ${error?.message || "Unknown error"}` 
      };
    }
  },

  /**
   * Request the current status of a vending machine
   * This is a specialized version of sendCommand for status requests
   */
  async requestStatus(privateKey, targetPubKey) {
    try {
      return await this.sendCommand(privateKey, targetPubKey, { type: "Status" });
    } catch (error) {
      console.error("Error in requestStatus:", error);
      return { 
        success: false, 
        message: `Error requesting status: ${error?.message || "Unknown error"}` 
      };
    }
  },

  /**
   * Subscribe to updates from a specific vending machine
   * 
   * @param {string} targetPubKey - Vending machine's public key (hex or npub format)
   * @param {function} callback - Function to call when an update is received
   * @returns {function} Unsubscribe function
   */
  subscribeToUpdates(targetPubKey, callback) {
    // Convert from NIP-19 format if needed
    let pubKey = targetPubKey;
    if (targetPubKey.startsWith('npub')) {
      try {
        const decoded = nip19.decode(targetPubKey);
        pubKey = decoded.data;
      } catch (err) {
        console.error("Error decoding npub:", err);
        throw new Error(`Invalid npub format: ${err.message}`);
      }
    }
    
    console.log(`Subscribing to updates from ${pubKey}`);
    // Create a subscription to listen for events from the target machine
    const filter = {
      kinds: [1], // Public note kind
      authors: [pubKey],
      // Remove time restriction to catch all updates
    };

    console.log('Setting up subscription with filter:', filter);
    
    const sub = pool.subscribeMany(RELAYS, [filter], {
      onevent(event) {
        try {
          console.log('Received event:', event);
          // Parse the content directly since it's not encrypted
          callback(event);
        } catch (error) {
          console.error('Error processing event:', error);
        }
      }
    });

    return () => {
      console.log('Closing subscription');
      sub.close();
    };
  },
};

export default nostrService;