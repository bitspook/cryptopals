import "highlight.js/styles/stackoverflow-light.css";
import { Runtime, Inspector } from "@observablehq/runtime";
import "@observablehq/inspector/dist/inspector.css";

import init, * as cryptopals from "~/pkg/cryptopals";
import "./components/PlayFunction";

// @ts-ignore - otherwise it give warning about import.meta
const athletes = new URL("./athletes.csv", import.meta.url);

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

const independentNb = () => {
  const runtime = new Runtime();

  const main = runtime.module();
  const inspector = new Inspector(document.getElementById("notebook"));
  const fileAttachments = new Map([["athletes.csv", athletes]]);

  main.builtin(
    "Attachment",
    runtime.fileAttachments((name) => fileAttachments.get(name))
  );

  main.variable(inspector).define("title", ["html"], function (html) {
    return html`<h1>HELLO FROM HTML</h1>`;
  });

  main
    .variable(inspector)
    .define("athletes", ["Attachment"], function (FileAttachment) {
      return FileAttachment("athletes.csv").csv({ typed: true });
    });

  main.variable(inspector).define(["dotplot"], function (dotplot) {
    return dotplot.legend("color");
  });
};

const runObservableNb = () => {};

const run = async () => {
  try {
    await init();
    (window as any).cryptopals = cryptopals;
    console.info("Successfully initiated rust WASM bindings.");
  } catch (err) {
    console.error("Failed to initialize WASM. Not rendering any widgets.");
  }

  setupCollapsibleBlocks();

  runObservableNb();
};

run();
