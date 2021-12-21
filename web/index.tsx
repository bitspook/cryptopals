import hljs from "highlight.js/lib/core";
import hljsRust from "highlight.js/lib/languages/rust";
import "highlight.js/styles/stackoverflow-light.css";

import init, * as cryptopals from "~/pkg/cryptopals";
import "./components/PlayFunction";

hljs.registerLanguage("rust", hljsRust);

const syntaxHlSrcBlocks = () => {
  document.querySelectorAll(".src").forEach((el) => {
    hljs.highlightElement(el as HTMLElement);
  });
};

const run = async () => {
  try {
    await init();
    (window as any).cryptopals = cryptopals;
    console.info("Successfully initiated rust WASM bindings.");
  } catch (err) {
    console.error("Failed to initialize WASM. Not rendering any widgets.");
  }

  syntaxHlSrcBlocks();
};

run();
