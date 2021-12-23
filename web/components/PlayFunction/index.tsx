import { useState, useCallback, useReducer, useEffect } from "preact/hooks";
import register from "preact-custom-element";

import * as s from "./style.module.scss";

import TransitionRightIcon from "~/web/components/icons/TransitionRight";
import { getWindowProp, classnames } from "~/web/utils";
import { renderFunctionInput } from "../FunctionInputs";

const c = classnames(s);

interface HexToB64Props {
  fn: string;
  displayName: string;
  errorMessage?: string;
  args?: string;
}

const inputsReducer = (inputs: string[], { index, value }) => {
  const nextInputs = [...inputs];
  nextInputs[index] = value;

  return nextInputs;
};

const HexToB64 = ({
  fn,
  displayName,
  errorMessage = "Invalid input",
  args = "string",
}: HexToB64Props) => {
  const [output, setOutput] = useState("");
  const [error, setError] = useState("");
  const [inputs, addInput] = useReducer(inputsReducer, []);

  const Args = args.split(",").map(renderFunctionInput);

  const executioner = getWindowProp(fn); // Read the function from window

  const handleChangeArg = useCallback((value: any, index: number) => {
    addInput({ index, value });
  }, []);

  useEffect(() => {
    if (typeof executioner !== "function") {
      return;
    }

    console.warn("Trying to eval", displayName, "with inputs: ", inputs);
    try {
      let output = executioner(...inputs);
      setOutput(output);
      setError("");
    } catch (err) {
      console.error(`Error while playing ${displayName}: `, err);
      setOutput("");
      setError(errorMessage);
    }
  }, [inputs, setOutput, executioner, errorMessage]);

  return (
    <div className={s.container}>
      <div className={s.input}>
        <div className={s.preInput + " hljs-title function_"}>
          {displayName}(
        </div>
        {Args.map((Arg, index) => (
          <Arg
            onChange={handleChangeArg}
            index={index}
            maxIndex={Args.length - 1}
          />
        ))}
        <div className={s.postInput + " hljs-title function_"}>)</div>
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

register(HexToB64, "play-function", [
  "fn",
  "display-name",
  "error-message",
  "args",
]);

export default HexToB64;
