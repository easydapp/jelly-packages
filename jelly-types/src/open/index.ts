import { array2hex, hex2array } from './open-hex';
import { ext_identifier2index, ext_index2identifier, principal2account_id } from './open-ic';
import { parse_factory, stringify_factory } from './open-json';
import { format_integer, format_number, unit } from './open-number';
import { link_value_to_js_value } from './open-type';
import { Principal } from './principal';

export default {
    // tools
    OpenJSON: { parse_factory, stringify_factory },
    OpenType: { link_value_to_js_value },
    OpenNumber: { format_number, format_integer, unit },
    OpenHex: { hex2array, array2hex },

    // ic
    Principal,
    OpenIc: { principal2account_id, ext_index2identifier, ext_identifier2index },
};
