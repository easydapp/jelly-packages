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

// only eval
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
export const doExecuteByEvalToModuleDirectly = async (
    code: string,
    args: [string, any][],
    debug: boolean,
): Promise<any> => {
    // 1. code
    let full_code = `export const main = (env, data) => {
  const mask = {};
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

  return inner.call(mask, ...data);
};`;

    if (debug) {
        console.log('full_code:');
        console.log(full_code);
    }

    // 2. module
    full_code = `data:text/javascript;charset=utf-8,${encodeURIComponent(full_code)}`;
    const module = await eval(`import("${full_code}")`); // parse to js module // ! warning: code is wrong, can not be module

    if (debug) console.log('module:', module);

    // 3. execute and get value
    const value = module.main(
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
