import { Principal } from '@dfinity/principal';

// Principal -> string
export const principal2string = (p: Principal): string => p.toText();

// string -> Principal // ! Note exception
export const string2principal = (p: string): Principal => Principal.fromText(p);
