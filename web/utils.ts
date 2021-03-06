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
    const reducer = (cns: SwitchableClassNames = {}, module?: Object) =>
      Object.entries(cns).reduce((finalName, [name, shouldAdd]) => {
        if (shouldAdd) {
          const cName = module ? module[name] : name;
          finalName += cName ? ` ${cName}` : '';
        }

        return finalName;
      }, "");

    const className =
      reducer(moduleNames, cssModule) + " " + reducer(nonModuleNames);

    return className;
  };
