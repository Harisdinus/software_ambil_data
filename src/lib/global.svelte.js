import { invoke } from '@tauri-apps/api/core';

export const globals = $state({
  url: "http://localhost:3000",
})

export class Global {
  static setUrl() {
    Global.fetch("set_url", { url: globals.url })
  }

  static async fetch(cmd, data) {
    try {
      return await invoke(cmd, data);
    } catch (err) {
      console.error(err)
    }
  }
}

