import { useState, useCallback } from "preact/hooks";
import register from "preact-custom-element";

import * as s from "./style.module.scss";

import TransitionRightIcon from "~/web/components/icons/TransitionRight";
import { getWindowProp, classnames } from "~/web/utils";

const c = classnames(s);

interface HexToB64Props {
  fn: string;
  displayName: string;
  errorMessage: string;
}

const HexToB64 = ({ fn, displayName, errorMessage }: HexToB64Props) => {
  const [output, setOutput] = useState("");
  const [error, setError] = useState("");

  const exec = getWindowProp(fn); // Read the function from window

  const handleChangeInput = useCallback(
    (el) => {
      const input = el.target.value;
      try {
        let b64 = exec(input);
        setOutput(b64);
        setError("");
      } catch (err) {
        console.error(`Error while playing ${displayName}: `, err);
        setOutput("");
        setError(errorMessage);
      }
    },
    [setOutput, exec, errorMessage]
  );

  return (
    <div className={s.container + " hljs"}>
      <div className={s.input}>
        <div className={s.preInput + " hljs-title function_"}>
          {displayName}("
        </div>
        <textarea
          onKeyup={handleChangeInput}
          placeholder="Enter HEX input here"
        ></textarea>
        <div className={s.postInput + " hljs-title function_"}>")</div>
      </div>

      <TransitionRightIcon
        {...c({
          transitioner: true,
          hasSuccess: Boolean(output),
          hasError: Boolean(error),
        })}
      />

      <div
        {...c({
          output: true,
          hasError: Boolean(error),
        })}
      >
        {output}

        {error && <div className={s.postOutput}>{error}</div>}
      </div>
    </div>
  );
};

register(HexToB64, "play-function", ["fn", "display-name", "error-message"]);

export default HexToB64;
