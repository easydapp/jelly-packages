// Code parameter constraint type
export interface CodeType {
    ty: string; // TS constraint type
    types?: string[]; // If there is a separate type constraint
}

// Parameters support multiple
export interface ArgCodeType {
    name: string; // Parameter alias
    ty: CodeType; // Parameter constraint
}

// Code with constraints
export interface CodeItem {
    code: string; // TS code
    args?: ArgCodeType[]; // Parameter type Parameters support multiple, so it needs to be named
    ret?: CodeType; // The only type
}
