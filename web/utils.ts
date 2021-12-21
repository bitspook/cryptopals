export const getWindowProp = (selector: string) =>
  selector.split(".").reduce((accum, prop) => {
    return accum && accum[prop];
  }, window);

type SwitchableClassNames = { [key: string]: boolean };

export const classnames =
  (cssModule: { [name: string]: string }) =>
  (
    moduleNames: SwitchableClassNames,
    nonModuleNames?: SwitchableClassNames
  ) => {
    const reducer = (cns: SwitchableClassNames, module?: Object) =>
      Object.entries(moduleNames).reduce((finalName, [name, shouldAdd]) => {
        if (shouldAdd) {
          finalName += ` ${module ? module[name] : name}`;
        }

        return finalName;
      }, "");

    const className =
      reducer(moduleNames, cssModule) + " " + reducer(nonModuleNames);

    return { className };
  };
