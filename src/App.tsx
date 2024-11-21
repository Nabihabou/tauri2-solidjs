import { Match, Switch } from "solid-js";
import "./App.css";
import { BridgeStatus } from "./lib/bridge";
import { ContextProvider, useCtx } from "./lib/ctx";

function App() {
  const { bridge } = useCtx();
  console.log(bridge.status[0]())
  
  const bridgeStatus = bridge.status[0]();

  console.log(bridgeStatus)

  return (
    <main class="container">
      <ContextProvider>
        <Switch fallback={<div>Not Found</div>}>
          <Match when={bridgeStatus === BridgeStatus.Authenticated}>
            authenticated
          </Match>
          <Match when={bridgeStatus === BridgeStatus.Connected}>
            connected
          </Match>
          <Match when={bridgeStatus === BridgeStatus.Disconnected}>
            disconnected

            <button onClick={() => bridge.connect()}>Connect</button>
          </Match>
          <Match when={bridgeStatus === BridgeStatus.Loading}>
            loading
          </Match>
        </Switch>
      </ContextProvider>
    </main>
  );
}

export default App;
