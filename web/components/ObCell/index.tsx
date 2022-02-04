import { useCallback, useEffect, useRef, useState } from "preact/hooks";
import { Inspector } from "@observablehq/runtime";

import { Interpreter } from "~/web/obhq-compiler/Interpreter";
import { classnames } from "~/web/utils";
import * as s from "./style.module.scss";

const c = classnames(s);

interface ObCellProps {
  id: string;
  module: any;                  // Name of the Observable module stored in `moduleRepo`
}

const ObCell =
  (modulesRepo: {[name: string]: any}) =>
  (p: ObCellProps) => {
    let observer, interpreter;

    // We need to do these shenanigans instead of obtaining a module like a
    // normal component because web-components can pass only strings as
    // attributes.
    const module = modulesRepo[p.module];

    if (!module) {
      console.error('Invalid or missing Observable module for ObCell');
      return null;
    }

    const codeEl = useRef(null);
    const outputEl = useRef(null);

    const [hasAttention, setHasAttention] = useState(false);
    const [isEditing, setIsEditing] = useState(false);
    const [code, setCode] = useState("");
    // Let's keep the HTML provided by org-mode, and use that for editing.
    // This saves us from adding a dependency on codemirror or something for now.
    const [codeHtml, setCodeHtml] = useState("");

    const cellNode = document.querySelector(`#${p.id}`);
    const codeNode = cellNode.firstChild;
    if (!code) setCode(codeNode.textContent);
    if (!codeHtml) setCodeHtml((codeNode as any).innerHTML);

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
      if (!codeEl.current) return;

      codeEl.current.innerHTML = codeHtml;
    }, [codeEl, codeHtml]);

    if (!cellNode) {
      console.error(
        "Unable to find cell-node for OBCell. [selector=",
        p.id,
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
