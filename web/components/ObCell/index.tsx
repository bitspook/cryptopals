import register from "preact-custom-element";
import { useEffect, useRef } from "preact/hooks";
import { Inspector } from "@observablehq/runtime";
import { Interpreter } from "@alex.garcia/unofficial-observablehq-compiler";

import { classnames } from "~/web/utils";
import * as s from "./style.module.scss";

const c = classnames(s);

interface ObCellProps {
  selector: string;
}

const ObCell =
  ({ module }) =>
  (p: ObCellProps) => {
    const codeEl = useRef(null);
    const outputEl = useRef(null);
    const cellNode = document.querySelector(p.selector);

    if (!cellNode) {
      console.error(
        "Unable to find cell-node for OBCell. [selector=",
        p.selector,
        "]"
      );
      return null;
    }

    const codeNode = cellNode.firstChild;
    const code = codeNode.textContent;

    useEffect(() => {
      if (!outputEl.current) return;

      const observer = Inspector.into(outputEl.current);

      const interpreter = new Interpreter({ module, observer });

      interpreter.module(code);
    }, [outputEl]);

    // Move the code element added by org-mode into our component for
    // easier manipulation
    useEffect(() => {
      if (!codeEl.current || codeEl.current === codeNode) return;
      codeNode.remove();

      codeEl.current.replaceWith(codeNode);
      codeEl.current = codeNode;
    }, [codeEl]);

    return (
      <div className={s.container}>
        <div className={s.gutter}></div>
        <div className={s.content}>
          <div className={s.outputContainer} ref={outputEl}></div>
          <div ref={codeEl}></div>
        </div>
      </div>
    );
  };

export default ObCell;
