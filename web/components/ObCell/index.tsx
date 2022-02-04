import { useCallback, useEffect, useRef, useState } from "preact/hooks";
import { Inspector } from "@observablehq/runtime";

import { Interpreter } from "~/web/obhq-compiler/Interpreter";
import { classnames } from "~/web/utils";
import * as s from "./style.module.scss";

const c = classnames(s);

interface ObCellProps {
  selector: string;
}

const ObCell =
  ({ module }) =>
  (p: ObCellProps) => {
    let observer, interpreter;

    const codeEl = useRef(null);
    const outputEl = useRef(null);

    const [hasAttention, setHasAttention] = useState(false);
    const [isEditing, setIsEditing] = useState(false);
    const [code, setCode] = useState("");

    const cellNode = document.querySelector(p.selector);
    const codeNode = cellNode.firstChild;
    if (!code) setCode(codeNode.textContent);

    const handleMouseEnter = useCallback(() => {
      setHasAttention(true);
    }, []);

    const handleMouseLeave = useCallback(() => {
      setHasAttention(false);
    }, []);

    const handleFocus = useCallback(() => {
      setIsEditing(true);
    }, []);

    const handleBlur = useCallback(() => {
      setIsEditing(false);
    }, []);

    const handleCodeChange = useCallback(() => {
      if (!codeEl.current) return;

      const newCode = codeEl.current.textContent;

      if (newCode === code) return;
      setCode(newCode);
    }, [codeEl, code]);

    useEffect(() => {
      if (!outputEl.current) return;

      if (!observer) observer = Inspector.into(outputEl.current);
      if (!interpreter) interpreter = new Interpreter({ module, observer });

      (window as any).interpreter = interpreter;

      if (!code) return;

      interpreter.module(code);
    }, [outputEl, code]);

    // Move the code element added by org-mode into our component for
    // easier manipulation
    useEffect(() => {
      if (!codeEl.current || codeEl.current === codeNode) return;
      codeNode.remove();

      codeEl.current.appendChild(codeNode);
    }, [codeEl]);

    if (!cellNode) {
      console.error(
        "Unable to find cell-node for OBCell. [selector=",
        p.selector,
        "]"
      );
      return null;
    }

    return (
      <div
        className={c({ container: true, hasAttention, isEditing })}
        onMouseEnter={handleMouseEnter}
        onMouseLeave={handleMouseLeave}
        onFocus={handleFocus}
      >
        <div className={s.gutter}></div>
        <div className={s.menu}></div>
        <div className={s.content}>
          <div className={s.outputContainer} ref={outputEl}></div>
          <div
            className={s.codeContainer}
            ref={codeEl}
            contentEditable
            spellcheck={false}
            onFocus={handleFocus}
            onBlur={handleBlur}
            onKeyUp={handleCodeChange}
          ></div>
        </div>
      </div>
    );
  };

export default ObCell;
