import saferEval from 'safer-eval';
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

// SaferEval and Function
// import 'xxx'; Cannot use import statement outside a module
// require('xxx'); require is not defined
// x document
// x window
// x global
// x globalThis
// x this
// x localStorage
// x sessionStorage
// x indexedDB
// x location
// x Function
// x eval
// x XMLHttpRequest
export const doExecuteBySaferEvalDirectly = async (
    code: string,
    args: [string, any][],
    debug: boolean,
): Promise<any> => {
    // 1. inner code
    const inner = `let result = undefined;
with(this) {
  ${code}
}
return result;`;

    if (debug) {
        console.log('inner:');
        console.log(inner);
    }

    // 2. outer code
    code = `((data) => {
  const mask = {};
  for (const p in this) mask[p] = undefined;

  ${MASKS.map(([key, value]) => `mask['${key}'] = ${value};`).join('\n  ')}

  ${MASKS.filter(([key]) => !EXCLUDES.includes(key))
      .map(([key, value]) => `let ${key} = ${value};`)
      .join('\n  ')}

  const func = (${args.map((a) => `${a[0]}`).join(', ')}) => {
    ${inner.split('\n').join('\n    ')}
  };

  return func.call(mask, ...data);
}).call({}, data)`;

    if (debug) {
        console.log('code:');
        console.log(code);
    }

    // 3. execute and get value
    const value = saferEval(code, {
        data: args.map((a) => a[1]), // Parameter data
        Function: Function, // function

        env: {
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
    }); // ! warning: code is wrong, execute error

    return value;
};
