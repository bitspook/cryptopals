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
    const outputEl = useRef(null);
    const code = document.querySelector(p.selector).textContent;

    useEffect(() => {
      if (!outputEl.current) return;

      const observer = Inspector.into(outputEl.current);

      const interpreter = new Interpreter({ module, observer });

      interpreter.module(code);
    }, [outputEl]);

    return (
      <div className={s.container}>
        <div ref={outputEl}></div>
      </div>
    );
  };

export default ObCell;
