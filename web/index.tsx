import "highlight.js/styles/stackoverflow-light.css";

import init, * as cryptopals from "~/pkg/cryptopals";
import "./components/PlayFunction";

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

const run = async () => {
  try {
    await init();
    (window as any).cryptopals = cryptopals;
    console.info("Successfully initiated rust WASM bindings.");
  } catch (err) {
    console.error("Failed to initialize WASM. Not rendering any widgets.");
  }

  setupCollapsibleBlocks();
};

run();
