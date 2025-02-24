export const parse_custom_style = <T>(customStyle: string | undefined): T | undefined => {
    return customStyle ? JSON.parse(customStyle) : undefined;
};
