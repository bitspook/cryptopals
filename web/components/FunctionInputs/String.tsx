import { useCallback } from "preact/hooks";
import * as s from "./styles.module.scss";

interface Props {
  placeholder?: string;
  index: number;
  maxIndex: number;
  onChange: (event, index: number) => void;
}

const FunctionInputString = ({
  onChange,
  index,
  maxIndex,
  placeholder = "Enter string",
}: Props) => {
  const handleChange = useCallback(
    (e) => {
      onChange(e, index);
    },
    [onChange]
  );

  return (
    <div className={s.inlineContainer}>
      <span className="hljs-title function_">"</span>
      <input
        type="text"
        className={s.input}
        onKeyup={handleChange}
        placeholder={placeholder}
      ></input>
      <span className="hljs-title function_">
        "{index !== maxIndex ? "," : null}
      </span>
    </div>
  );
};

export default FunctionInputString;
