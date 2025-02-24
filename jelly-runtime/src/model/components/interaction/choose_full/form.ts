import { ValidateForm } from '../../../common/validate';

export interface ChooseFullForm {
    default?: string; // Default value The default value of the input box. Because there is a "Trigger" button, the default here is only an auxiliary filling. The confirmation button can only be triggered
    confirm?: string;
    validate?: ValidateForm;
}
