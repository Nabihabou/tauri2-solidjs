import { createStore } from "solid-js/store";

interface State {
  bridge: {
    status: "connected" | "disconnected" | "loading" | "authenticated";
  };
}

export const [state, setState] = createStore<State>({
  bridge: {
    status: "disconnected",
  },
});
