import { InternetComputerApi, match_internet_computer_api_async } from '../../../../../../store/api/content/ic';
import { WrappedCandidTypeFunction } from '../../../../../../wasm/candid';
import { parse_func_candid_by_remote, parse_service_candid_by_remote } from './candid_by_remote';

export const parse_ic_api = async (
    api: InternetComputerApi,
    handle: (_method_: string | undefined, _func_: WrappedCandidTypeFunction | undefined) => void,
    parse_service_candid = parse_service_candid_by_remote,
    parse_func_candid = parse_func_candid_by_remote,
) => {
    let _method_: string | undefined = undefined;
    let _func_: WrappedCandidTypeFunction | undefined = undefined;

    await match_internet_computer_api_async(api, {
        single: async (single) => {
            const [name, wrapped_func] = await parse_func_candid(single.api, (s) => s, false);
            _method_ = name;
            _func_ = wrapped_func;
        },
        origin: async (origin) => {
            const service = await parse_service_candid(origin.candid, (s) => s, false);
            const methods = service.methods ?? [];
            _method_ = origin.method;
            _func_ = methods.find((item) => item[0] === _method_)?.[1];
        },
    });

    handle(_method_, _func_);
};

// console.debug(
//     'call ic func arg',
//     func.argTypes.map((s) => s.name),
// );
// console.debug(
//     'call ic func ret',
//     func.retTypes.map((s) => s.name),
// );

// const js = match_internet_computer_api<string>(api, {
//     single: (single) => single.js,
//     origin: (origin) => origin.js,
// });
// const code = `data:text/javascript;charset=utf-8,${encodeURIComponent(js)}`;
// const idl2 = await eval(`import("${code}")`);
// const methodList = await parseOptions(idl2, canister_id);
// console.debug(`🚀 ~ call: ~ methodList:`, methodList, idl2);
// const func2: any = methodList.find((item) => item.name === method)!.func;
// console.debug(
//     'call ic func2 arg',
//     func2.argTypes.map((s: any) => s.name),
// );
// console.debug(
//     'call ic func2 ret',
//     func2.retTypes.map((s: any) => s.name),
// );
// let actor: any = undefined;
// try {
//     actor = await identity_metadata.creator(idl2.idlFactory, canister_id); // Get ACTOR
// } catch (e) {
//     throw handle_error(identity_metadata, e);
// }
