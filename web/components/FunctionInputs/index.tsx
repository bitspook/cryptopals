import FunctionInputInvalid from "./Invalid";
import FunctionInputString from "./String";

export const renderFunctionInput = (argType: string) => {
  switch (argType.trim()) {
    case "string":
      return FunctionInputString;
    default:
      return FunctionInputInvalid(argType);
  }
};
