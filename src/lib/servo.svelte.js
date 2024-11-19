import { invoke } from '@tauri-apps/api/core';

export class Servo {
  content = $state("");
  error = $state("");

  async up() {
    this.fetch("servo_up");
  }

  async down() {
    this.fetch("servo_down");
  }

  async reset() {
    this.fetch("servo_reset");
  }

  async fetch(cmd) {
    try {
      this.content = await invoke(cmd);
    } catch (err) {
      this.error = "Error: " + err.message
      console.error(err)
    }
  }
}


