import "highlight.js/styles/stackoverflow-light.css";
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

type ObModule = any;

const initializeObNotebook = () => {
  const registerModule = (mName: string, module: ObModule) => {
    const ModuleObCell = ObCell({ module });

    register(ModuleObCell, `ob-cell-${mName}`, ["selector"]);
  };

  const runtime = new Runtime();
  const modules = {
    main: runtime.module(),
  };

  Object.entries(modules).forEach(([mName, module]) =>
    registerModule(mName, module)
  );

  document.querySelectorAll(".ob-cell").forEach((el) => {
    if (!el.id) {
      console.error(
        "ob-cell must be uniquely identifiable. Please add an id for",
        el
      );
      return;
    }

    const mName = el.getAttribute("data-module") || "main";

    if (!modules[mName]) {
      const module = runtime.module();
      modules[mName] = module;
      registerModule(mName, module);
    }

    const cellEl = document.createElement(`ob-cell-${mName}`);
    cellEl.setAttribute("selector", `#${el.id}`);
    el.appendChild(cellEl);
  });
};

const run = async () => {
  await initializeWasm();
  setupCollapsibleBlocks();
  initializeObNotebook();
};

run();
