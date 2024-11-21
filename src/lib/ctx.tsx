import { createContext, useContext } from "solid-js";
import { Bridge } from "./bridge";

const CTX = {
  bridge: new Bridge(),
};

const Context = createContext(CTX);

export function useCtx() {
  return useContext(Context);
}

export function ContextProvider(props: any) {
  return (
    <Context.Provider value={CTX}>
      {props.children}
    </Context.Provider>
  )
}
