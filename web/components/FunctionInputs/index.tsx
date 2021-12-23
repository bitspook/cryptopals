import FunctionInputInvalid from "./Invalid";
import FunctionInputString from "./String";
import FunctionInputNumber from "./Number";

export const renderFunctionInput = (argType: string) => {
  switch (argType.trim()) {
    case "string":
      return FunctionInputString;
    case "number":
      return FunctionInputNumber;
    default:
      return FunctionInputInvalid(argType);
  }
};
