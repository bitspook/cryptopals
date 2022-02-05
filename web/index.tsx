import { Runtime, Inspector } from "@observablehq/runtime";
import "@observablehq/inspector/dist/inspector.css";
import register from "preact-custom-element";

import init, * as cryptopals from "~/pkg/cryptopals";
import "./components/PlayFunction";
import ObCell from "./components/ObCell";

const setupCollapsibleBlocks = () => {
  document.querySelectorAll(".reveal").forEach((el) => {
    const classToReveal = el.getAttribute("data-reveal");
    const elsToReveal = document.querySelectorAll(`.${classToReveal}`);

    elsToReveal.forEach((elToReveal) => elToReveal.classList.add("collapsed"));

    el.addEventListener("click", (clickedEl) => {
      const isRevealed = el.classList.contains("revealed");

      if (isRevealed) {
        el.classList.remove("revealed");
      } else {
        el.classList.add("revealed");
      }

      elsToReveal.forEach((elToReveal) => {
        if (isRevealed) {
          elToReveal.classList.add("collapsed");
        } else {
          elToReveal.classList.remove("collapsed");
        }
      });
    });
  });
};

const initializeWasm = async () => {
  try {
    await init();
    (window as any).cryptopals = cryptopals;

    console.info("Successfully initiated rust WASM bindings.");
  } catch (err) {
    console.error("Failed to initialize WASM. Not rendering any widgets.");
  }
};

const initializeObNotebook = () => {
  const runtime = new Runtime();
  const modules = {
    main: runtime.module(),
  };

  // Create modules specified by <ob-cell>s
  document.querySelectorAll("ob-cell").forEach((el) => {
    const mName = el.getAttribute("module") || "main";

    if (!modules[mName]) {
      const module = runtime.module();
      modules[mName] = module;
    }
  });

  register(ObCell(modules), `ob-cell`, ["id", "module"]);
};

const run = async () => {
  await initializeWasm();
  setupCollapsibleBlocks();
  initializeObNotebook();
};

run();
