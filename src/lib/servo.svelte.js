import { invoke } from '@tauri-apps/api/core';

export class Servo {
  servo_up = $state(4)
  servo_down = $state(2)

  content = $state("")
  error = $state("")

  async up() {
    this.content = await this.fetch("servo_up", { value: this.servo_up })
  }

  async down() {
    this.content = await this.fetch("servo_down", { value: this.servo_down })
  }

  async reset() {
    await this.fetch("servo_reset")
  }

  async geterin() {
    await this.fetch("geterin")
  }

  async fetch(cmd, data) {
    try {
      return await invoke(cmd, data);
    } catch (err) {
      this.error = "Error: " + err.message
      console.error(err)
    }
  }
}


