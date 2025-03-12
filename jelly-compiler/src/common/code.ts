import { CodeItem } from '@jellypack/types';
import * as esbuild from 'esbuild-wasm';
import wasmURL from 'esbuild-wasm/esbuild.wasm?url';
import { Linter } from 'eslint-linter-browserify';
import _ from 'lodash';

// import { javascript, esLint, typescriptLanguage } from '@codemirror/lang-javascript';

// ? see https://github.com/hellof2e/quark-playground/blob/main/src/build.ts

let initializing: Promise<void>;

try {
    initializing = esbuild.initialize({ wasmURL });
} catch (e) {
    // noop
    console.debug(`🚀 ~ initializing esbuild:`, e);
}

export const compile_code = async (item: CodeItem, debug = false): Promise<string> => {
    await initializing;

    let ty_types = [];
    if (item.args) {
        for (const arg of item.args) {
            if (arg.ty.types) ty_types.push(...arg.ty.types);
        }
    }
    if (item.ret?.types) ty_types.push(...item.ret.types);

    let args = '';
    if (item.args) {
        args = item.args.map((arg) => `${arg.name}: ${arg.ty.ty}`).join(', ');
    }

    let ret = '';
    if (item.ret?.ty) {
        if (item.ret.ty !== 'Output' && !ty_types.find((x) => x.startsWith('type Output = '))) {
            ty_types.push(`type Output = ${item.ret?.ty};`);
            ret = `: Output | undefined`;
        } else {
            ret = `: ${item.ret.ty} | undefined`;
        }
    }

    ty_types = _.uniq(ty_types);
    const types = ty_types.join('\n');

    const code = `${types ? `${types}\n` : ''}export const main = (${args})${ret} => {
    let result${ret} = undefined;
    ${item.code}
    return result;
}`;

    if (debug) console.group();

    try {
        if (debug) console.debug('compile typescript code:');
        if (debug) console.debug(code);

        // const linter2 = new Linter({ configType: 'eslintrc' });
        // // const linter2 = new Linter({ configType: 'flat' });
        // // const linter2 = new Linter();
        // linter2.defineParser('', typescriptLanguage.parser);
        // const checked2 = linter2.verify(code, LINTER_CONFIG);
        // if (debug)console.debug(`🚀 ~ const compile_code= ~ checked2:`, checked2);

        // Compile and remove type constraints, restore into the original JS
        const result = await esbuild.transform(code, {
            loader: 'ts',
            target: 'esnext',
        });

        // if (debug)console.debug('compile_code:', result);
        if (debug) console.debug('compiled code:');
        if (debug) console.debug(result.code);

        // Verify the basic grammar
        const linter = new Linter();
        const checked: Linter.LintMessage[] = linter.verify(result.code, LINTER_CONFIG);
        const pass_error = (e: Linter.LintMessage): boolean => {
            // Allow users to return in advance
            if (e.ruleId === 'no-unreachable') {
                const lines = result.code.split('\n');
                if (debug) console.debug(`🚀 ~ const compile_code= ~ lines:`, lines);
                if (
                    e.line === lines.length - 2 && // The third line of the countdown, and the end of the end
                    e.endLine === lines.length - 2 &&
                    e.column === 3 &&
                    e.endColumn === 17
                ) {
                    return true;
                }
            }

            return false;
        };
        if (checked.filter((e) => !pass_error(e)).length) {
            console.error(`🚀 ~ const compile_code= ~ checked:`, checked);
            throw new Error(JSON.stringify(checked));
        }

        let compiled = result.code;
        // Remove the prefix // ! Note that the compiled code is 2 spaces
        const prefix = `let result = void 0;
`;
        const index = compiled.indexOf(prefix);
        if (compiled.indexOf(prefix) < 0) throw new Error('compile error prefix');
        compiled = compiled.substring(index + prefix.length);
        // if (debug)console.error('', JSON.stringify(compiled));
        // Remove the suffix // ! Note that the compiled code is 2 spaces
        const suffix = `
  return result;
};
`;
        if (!compiled.endsWith(suffix)) {
            if (compiled.endsWith(suffix.substring(1))) {
                compiled = compiled.substring(0, compiled.length - suffix.length - 1);
            } else {
                console.debug('compiled code: removed prefix');
                console.debug(compiled);
                throw new Error('compile error suffix');
            }
        } else {
            compiled = compiled.substring(0, compiled.length - suffix.length);
        }

        if (debug) console.debug('finial code:');
        if (debug) console.debug(compiled);

        return compiled.trim();
    } finally {
        if (debug) console.groupEnd();
    }
};

const LINTER_CONFIG: Linter.Config = {
    languageOptions: {
        globals: {
            OpenJSON: 'readonly', // ! Support the JSON tool provided in the code
            OpenType: 'readonly', // ! Support the type tools provided in the code
            OpenNumber: 'readonly', // ! Support the number formatting tool used in the code
            OpenHex: 'readonly', // ! Support code in the code to use the HEX formatting tool

            // ic
            Principal: 'readonly', // ! Support code in the code
            OpenIc: 'readonly', // ! Support the IC tool used in the code
        },
    },
    rules: {
        'accessor-pairs': ['error'],
        // 'array-bracket-newline': ['error'], // * Not required
        // 'array-bracket-spacing': ['error'], // * Not required
        'array-callback-return': ['error'],
        // 'array-element-newline': ['error'], // ? Style does not check
        // 'arrow-body-style': ['error'], // * Not required // Arrow function style
        'arrow-parens': ['error'],
        'arrow-spacing': ['error'],
        'block-scoped-var': ['error'],
        'block-spacing': ['error'],
        'brace-style': ['error'],
        'callback-return': ['error'],
        // camelcase: ['error'], // * Not required // cspell: disable-line
        'capitalized-comments': ['error'], // Power -written notes
        'class-methods-use-this': ['error'],
        'comma-dangle': ['error'],
        'comma-spacing': ['error'],
        'comma-style': ['error'],
        // complexity: ['error'], // * Not required // Arrow function complexity
        'computed-property-spacing': ['error'],
        'consistent-return': ['error'],
        'consistent-this': ['error'],
        'constructor-super': ['error'],
        // curly: ['error'], // * Not required // Brackets
        'default-case': ['error'],
        'default-case-last': ['error'],
        'default-param-last': ['error'],
        'dot-location': ['error'],
        'dot-notation': ['error'],
        // 'eol-last': ['error'], // ? Style does not check
        eqeqeq: ['error'],
        'for-direction': ['error'],
        'func-call-spacing': ['error'],
        'func-name-matching': ['error'],
        'func-names': ['error'],
        'func-style': ['error'],
        // 'function-call-argument-newline': ['error'], // * Not required // Function calls in Xinxing
        // 'function-paren-newline': ['error'], // deprecated
        'generator-star-spacing': ['error'],
        'getter-return': ['error'],
        'global-require': ['error'],
        'grouped-accessor-pairs': ['error'],
        'guard-for-in': ['error'],
        'handle-callback-err': ['error'],
        'id-blacklist': ['error'],
        'id-denylist': ['error'],
        // 'id-length': ['error'], // * Not required
        'id-match': ['error'],
        'implicit-arrow-linebreak': ['error'], // cspell: disable-line
        // indent: ['error'], // ? Style does not check
        // 'indent-legacy': ['error'], // ? Style does not check
        'init-declarations': ['error'],
        'jsx-quotes': ['error'],
        'key-spacing': ['error'],
        'keyword-spacing': ['error'],
        'line-comment-position': ['error'],
        'linebreak-style': ['error'], // cspell: disable-line
        'lines-around-comment': ['error'],
        'lines-around-directive': ['error'],
        'lines-between-class-members': ['error'],
        'logical-assignment-operators': ['error'],
        'max-classes-per-file': ['error'],
        'max-depth': ['error'],
        // 'max-len': ['error'], // ? Style does not check
        'max-lines': ['error'],
        // 'max-lines-per-function': ['error'], // * Not required
        'max-nested-callbacks': ['error'],
        // 'max-params': ['error'], // * Not required // The number of parameters of the arrow function
        // 'max-statements': ['error'], // * Not required // Coding habit
        'max-statements-per-line': ['error'],
        'multiline-comment-style': ['error'],
        // 'multiline-ternary': ['error'], // ? Style does not check
        'new-cap': ['error'],
        'new-parens': ['error'],
        // 'newline-after-var': ['error'], // ? Style does not check
        // 'newline-before-return': ['error'], // ? Style does not check
        'newline-per-chained-call': ['error'],
        'no-alert': ['error'],
        'no-array-constructor': ['error'],
        'no-async-promise-executor': ['error'],
        'no-await-in-loop': ['error'],
        'no-bitwise': ['error'],
        'no-buffer-constructor': ['error'],
        'no-caller': ['error'],
        'no-case-declarations': ['error'],
        'no-catch-shadow': ['error'],
        'no-class-assign': ['error'],
        'no-compare-neg-zero': ['error'],
        'no-cond-assign': ['error'],
        // 'no-confusing-arrow': ['error'], // * Not required // Confused arrow expression
        'no-console': ['error'],
        'no-const-assign': ['error'],
        'no-constant-binary-expression': ['error'],
        'no-constant-condition': ['error'],
        'no-constructor-return': ['error'],
        'no-continue': ['error'],
        'no-control-regex': ['error'],
        'no-debugger': ['error'],
        'no-delete-var': ['error'],
        'no-div-regex': ['error'],
        'no-dupe-args': ['error'],
        'no-dupe-class-members': ['error'],
        'no-dupe-else-if': ['error'],
        'no-dupe-keys': ['error'],
        'no-duplicate-case': ['error'],
        'no-duplicate-imports': ['error'],
        'no-else-return': ['error'],
        'no-empty': ['error'],
        'no-empty-character-class': ['error'],
        'no-empty-function': ['error'],
        'no-empty-pattern': ['error'],
        'no-empty-static-block': ['error'],
        'no-eq-null': ['error'],
        'no-eval': ['error'],
        'no-ex-assign': ['error'],
        'no-extend-native': ['error'],
        'no-extra-bind': ['error'],
        'no-extra-boolean-cast': ['error'],
        'no-extra-label': ['error'],
        'no-extra-parens': ['error'],
        'no-extra-semi': ['error'],
        'no-fallthrough': ['error'],
        'no-floating-decimal': ['error'],
        'no-func-assign': ['error'],
        'no-global-assign': ['error'],
        // 'no-implicit-coercion': ['error'], // * Not required // Invisible mandatory conversion
        'no-implicit-globals': ['error'],
        'no-implied-eval': ['error'],
        'no-import-assign': ['error'],
        'no-inline-comments': ['error'],
        'no-inner-declarations': ['error'],
        'no-invalid-regexp': ['error'],
        'no-invalid-this': ['error'],
        'no-irregular-whitespace': ['error'],
        'no-iterator': ['error'],
        'no-label-var': ['error'],
        'no-labels': ['error'],
        'no-lone-blocks': ['error'],
        'no-lonely-if': ['error'],
        'no-loop-func': ['error'],
        'no-loss-of-precision': ['error'],
        // 'no-magic-numbers': ['error'], // * The compiled code exists
        'no-misleading-character-class': ['error'],
        // 'no-mixed-operators': ['error'], // * Not required // Confused operating character priority
        'no-mixed-requires': ['error'],
        'no-mixed-spaces-and-tabs': ['error'],
        'no-multi-assign': ['error'],
        'no-multi-spaces': ['error'],
        'no-multi-str': ['error'],
        'no-multiple-empty-lines': ['error'],
        'no-native-reassign': ['error'],
        // 'no-negated-condition': ['error'], // * Not required // Negative expression
        'no-negated-in-lhs': ['error'],
        'no-nested-ternary': ['error'],
        'no-new': ['error'],
        'no-new-func': ['error'],
        'no-new-native-nonconstructor': ['error'], // cspell: disable-line
        'no-new-object': ['error'],
        'no-new-require': ['error'],
        'no-new-symbol': ['error'],
        'no-new-wrappers': ['error'],
        'no-nonoctal-decimal-escape': ['error'], // cspell: disable-line
        'no-obj-calls': ['error'],
        'no-object-constructor': ['error'],
        'no-octal': ['error'],
        'no-octal-escape': ['error'],
        // 'no-param-reassign': ['error'], // * Not required // Function parameters can be repaid again
        'no-path-concat': ['error'],
        // 'no-plusplus': ['error'], // * Not required // ++ // cspell: disable-line
        'no-process-env': ['error'],
        'no-process-exit': ['error'],
        'no-promise-executor-return': ['error'],
        'no-proto': ['error'],
        'no-prototype-builtins': ['error'],
        'no-redeclare': ['error'],
        'no-regex-spaces': ['error'],
        'no-restricted-exports': ['error'],
        'no-restricted-globals': ['error'],
        'no-restricted-imports': ['error'],
        'no-restricted-modules': ['error'],
        'no-restricted-properties': ['error'],
        'no-restricted-syntax': ['error'],
        'no-return-assign': ['error'],
        'no-return-await': ['error'],
        'no-script-url': ['error'],
        'no-self-assign': ['error'],
        'no-self-compare': ['error'],
        'no-sequences': ['error'],
        'no-setter-return': ['error'],
        'no-shadow': ['error'],
        'no-shadow-restricted-names': ['error'],
        'no-spaced-func': ['error'],
        'no-sparse-arrays': ['error'],
        'no-sync': ['error'],
        'no-tabs': ['error'],
        'no-template-curly-in-string': ['error'],
        // 'no-ternary': ['error'], // * Not required // Allow the three yuan computing formula
        'no-this-before-super': ['error'],
        'no-throw-literal': ['error'],
        'no-trailing-spaces': ['error'],
        'no-undef': ['error'],
        'no-undef-init': ['error'],
        'no-undefined': ['error'],
        // 'no-underscore-dangle': ['error'], // * Not required // Suspension lines appear in the identifier
        'no-unexpected-multiline': ['error'],
        'no-unmodified-loop-condition': ['error'],
        'no-unneeded-ternary': ['error'],
        'no-unreachable': ['error'],
        'no-unreachable-loop': ['error'],
        'no-unsafe-finally': ['error'],
        'no-unsafe-negation': ['error'],
        'no-unsafe-optional-chaining': ['error'],
        'no-unused-expressions': ['error'],
        'no-unused-labels': ['error'],
        'no-unused-private-class-members': ['error'],
        'no-use-before-define': ['error'],
        // 'no-useless-assignment': ['error'], // * The compiled code exists // Excess assignment
        'no-useless-backreference': ['error'], // cspell: disable-line
        'no-useless-call': ['error'],
        'no-useless-catch': ['error'],
        'no-useless-computed-key': ['error'],
        'no-useless-concat': ['error'],
        'no-useless-constructor': ['error'],
        'no-useless-escape': ['error'],
        'no-useless-rename': ['error'],
        'no-useless-return': ['error'],
        'no-var': ['error'],
        // 'no-void': ['error'], // * The compiled code exists
        'no-warning-comments': ['error'],
        'no-whitespace-before-property': ['error'],
        'no-with': ['error'],
        'nonblock-statement-body-position': ['error'], // cspell: disable-line
        'object-curly-newline': ['error'],
        // 'object-curly-spacing': ['error'], // * Not required // Object left flower bracket without space
        // 'object-property-newline': ['error'], // * Not required // Object field change line
        'object-shorthand': ['error'],
        // 'one-var': ['error'], // * Not required
        'one-var-declaration-per-line': ['error'],
        // 'operator-assignment': ['error'], // * Not required
        'operator-linebreak': ['error'], // cspell: disable-line
        // 'padded-blocks': ['error'], // ? Style does not check
        'padding-line-between-statements': ['error'],
        'prefer-arrow-callback': ['error'],
        // 'prefer-const': ['error'], // * Not required
        // 'prefer-destructuring': ['error'], // * Not required // Deconstruction expression
        'prefer-exponentiation-operator': ['error'],
        // 'prefer-named-capture-group': ['error'], // * Not required // Non -capture regular expression
        'prefer-numeric-literals': ['error'],
        'prefer-object-has-own': ['error'],
        'prefer-object-spread': ['error'],
        'prefer-promise-reject-errors': ['error'],
        'prefer-reflect': ['error'],
        'prefer-regex-literals': ['error'],
        'prefer-rest-params': ['error'],
        'prefer-spread': ['error'],
        // 'prefer-template': ['error'], // * Preference string template, programming preferences are not required
        // 'quote-props': ['error'], // * Not required // key Dual quotation
        // quotes: ['error'], // * Not required // Dual quotation
        // radix: ['error'], // * Not required // Advance
        'require-atomic-updates': ['error'],
        'require-await': ['error'],
        // 'require-unicode-regexp': ['error'], // * Not required // unicode There are too many regular requirements, and the compilation is not recognized，lint Identification is useless
        'require-yield': ['error'],
        'rest-spread-spacing': ['error'],
        semi: ['error'],
        'semi-spacing': ['error'],
        'semi-style': ['error'],
        'sort-imports': ['error'],
        // 'sort-keys': ['error'], // * Not required // Key sorting
        'sort-vars': ['error'],
        'space-before-blocks': ['error'],
        'space-before-function-paren': ['error'],
        'space-in-parens': ['error'],
        'space-infix-ops': ['error'],
        'space-unary-ops': ['error'],
        'spaced-comment': ['error'],
        strict: ['error'],
        'switch-colon-spacing': ['error'],
        'symbol-description': ['error'],
        'template-curly-spacing': ['error'],
        'template-tag-spacing': ['error'],
        'unicode-bom': ['error'],
        'use-isnan': ['error'],
        'valid-typeof': ['error'],
        'vars-on-top': ['error'],
        'wrap-iife': ['error'], // cspell: disable-line
        // 'wrap-regex': ['error'], // * Not required // The compilation will remove the outside brackets, always report an error
        'yield-star-spacing': ['error'],
        // yoda: ['error'], // * Not required // The literal can be on the left
    },
};
