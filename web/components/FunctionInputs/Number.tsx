import { useCallback } from "preact/hooks";
import * as s from "./styles.module.scss";

interface Props {
  placeholder?: string;
  index: number;
  maxIndex: number;
  onChange: (value: number, index: number) => void;
}

const FunctionInputNumber = ({
  onChange,
  index,
  maxIndex,
  placeholder = "Enter a number",
}: Props) => {
  const handleChange = useCallback(
    (e) => {
      onChange(Number(e.target.value), index);
    },
    [onChange]
  );

  return (
    <div className={s.inlineContainer}>
      <input
        type="number"
        className={s.input}
        onKeyup={handleChange}
        placeholder={placeholder}
      ></input>
    </div>
  );
};

export default FunctionInputNumber;
