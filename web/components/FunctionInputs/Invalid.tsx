import * as s from "./styles.module.scss";

const FunctionInputInvalid = (argType: string) => () => {
  return <div className={s.error}>Invalid Argument type: ${argType}</div>;
};

export default FunctionInputInvalid;
