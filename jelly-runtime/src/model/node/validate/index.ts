import { NodeTemplateValidateArray } from './array';
import { NodeTemplateValidateBool } from './bool';
import { NodeTemplateValidateEvmAddress } from './evm_address';
import { NodeTemplateValidateHex } from './hex';
import { NodeTemplateValidateImage } from './image';
import { NodeTemplateValidateInteger } from './integer';
import { NodeTemplateValidateNumber } from './number';
import { NodeTemplateValidateObject } from './object';
import { NodeTemplateValidatePrincipal } from './principal';
import { NodeTemplateValidateText } from './text';

export type NodeTemplateValidate =
    | { text: NodeTemplateValidateText }
    | { image: NodeTemplateValidateImage }
    | { principal: NodeTemplateValidatePrincipal }
    | { evm_address: NodeTemplateValidateEvmAddress }
    | { hex: NodeTemplateValidateHex }
    | { bool: NodeTemplateValidateBool }
    | { integer: NodeTemplateValidateInteger }
    | { number: NodeTemplateValidateNumber }
    | { array: NodeTemplateValidateArray }
    | { object: NodeTemplateValidateObject };

export const match_template_validate = <T>(
    self: NodeTemplateValidate,
    {
        text,
        image,
        principal,
        evm_address,
        hex,
        bool,
        integer,
        number,
        array,
        object,
    }: {
        text: (text: NodeTemplateValidateText) => T;
        image: (image: NodeTemplateValidateImage) => T;
        principal: (principal: NodeTemplateValidatePrincipal) => T;
        evm_address: (evm_address: NodeTemplateValidateEvmAddress) => T;
        hex: (hex: NodeTemplateValidateHex) => T;
        bool: (hex: NodeTemplateValidateBool) => T;
        integer: (integer: NodeTemplateValidateInteger) => T;
        number: (number: NodeTemplateValidateNumber) => T;
        array: (number: NodeTemplateValidateArray) => T;
        object: (object: NodeTemplateValidateObject) => T;
    },
): T => {
    if ('text' in self) return text(self.text);
    if ('image' in self) return image(self.image);
    if ('principal' in self) return principal(self.principal);
    if ('evm_address' in self) return evm_address(self.evm_address);
    if ('hex' in self) return hex(self.hex);
    if ('bool' in self) return bool(self.bool);
    if ('integer' in self) return integer(self.integer);
    if ('number' in self) return number(self.number);
    if ('array' in self) return array(self.array);
    if ('object' in self) return object(self.object);
    throw new Error('not support');
};

export const match_template_validate_async = async <T>(
    self: NodeTemplateValidate,
    {
        text,
        image,
        principal,
        evm_address,
        hex,
        bool,
        integer,
        number,
        array,
        object,
    }: {
        text: (text: NodeTemplateValidateText) => Promise<T>;
        image: (image: NodeTemplateValidateImage) => Promise<T>;
        principal: (principal: NodeTemplateValidatePrincipal) => Promise<T>;
        evm_address: (evm_address: NodeTemplateValidateEvmAddress) => Promise<T>;
        hex: (hex: NodeTemplateValidateHex) => Promise<T>;
        bool: (hex: NodeTemplateValidateBool) => Promise<T>;
        integer: (integer: NodeTemplateValidateInteger) => Promise<T>;
        number: (number: NodeTemplateValidateNumber) => Promise<T>;
        array: (number: NodeTemplateValidateArray) => Promise<T>;
        object: (object: NodeTemplateValidateObject) => Promise<T>;
    },
): Promise<T> => {
    if ('text' in self) return text(self.text);
    if ('image' in self) return image(self.image);
    if ('principal' in self) return principal(self.principal);
    if ('evm_address' in self) return evm_address(self.evm_address);
    if ('hex' in self) return hex(self.hex);
    if ('bool' in self) return bool(self.bool);
    if ('integer' in self) return integer(self.integer);
    if ('number' in self) return number(self.number);
    if ('array' in self) return array(self.array);
    if ('object' in self) return object(self.object);
    throw new Error('not support');
};
