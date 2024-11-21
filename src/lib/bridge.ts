import { invoke } from "@tauri-apps/api/core";
import { createSignal } from "solid-js";

export enum BridgeStatus {
  Disconnected = "disconnected",
  Loading = "loading",
  Connected = "connected",
  Authenticated = "authenticated",
}

// interface BridgeState {
//   status: BridgeStatus;
//   connect: () => void;
// }

export class Bridge {
  status = createSignal(BridgeStatus.Disconnected);

  async connect() {
    const status = await invoke("plugin:bridge|connect");
    this.status[1](status as BridgeStatus);
  }
}
