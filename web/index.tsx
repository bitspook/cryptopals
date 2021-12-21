import "highlight.js/styles/stackoverflow-light.css";

import init, * as cryptopals from "~/pkg/cryptopals";
import "./components/PlayFunction";

const run = async () => {
  try {
    await init();
    (window as any).cryptopals = cryptopals;
    console.info("Successfully initiated rust WASM bindings.");
  } catch (err) {
    console.error("Failed to initialize WASM. Not rendering any widgets.");
  }
};

run();
