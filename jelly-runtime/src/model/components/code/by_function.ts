import {
    EXCLUDES,
    MASKS,
    OpenHex,
    OpenIc,
    OpenJSON,
    OpenNumber,
    OpenType,
    Principal,
} from './common';

// only Function
// document
// window
// global
// globalThis
// x this
// localStorage
// sessionStorage
// indexedDB
// location
// Function
// eval
// XMLHttpRequest
export const doExecuteByFunctionDirectly = async (
    code: string,
    args: [string, any][],
    debug: boolean,
): Promise<any> => {
    // 1. code
    const inner = `  const mask = {};
  for (const p in this) mask[p] = undefined;

  ${MASKS.map(([key, value]) => `mask['${key}'] = ${value};`).join('\n  ')}

  ${MASKS.filter(([key]) => !EXCLUDES.includes(key))
      .map(([key, value]) => `let ${key} = ${value};`)
      .join('\n  ')}

  const inner = (${args.map((a) => a[0]).join(', ')}) => {
    let result = undefined;
    ${code.split('\n').join('\n  ')}
    return result;
  }

  return inner.call(mask, ...data);`;
    const func = new Function('env', 'data', inner);

    if (debug) {
        console.log('inner:');
        console.log(inner);
        console.log('func:', func);
    }

    // 2. execute and get value
    const value = func.call(
        {},
        {
            JSON: {
                stringify: JSON.stringify,
                parse: JSON.parse,
            },

            // tools
            OpenJSON,
            OpenType,
            OpenNumber, // Provide digital formatting tools
            OpenHex, // Provide hex tool

            // ic
            Principal, // Provide principal objects
            OpenIc, // Provide IC tools
        },
        args.map((a) => a[1]),
    ); // ! warning: code is wrong, execute error

    return value;
};
