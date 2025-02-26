import { LinkType } from '@jellypack/types/lib/types';

import { ComponentId } from '../../common/identity';
import { Endpoint } from '../../common/lets';
import { ValidateForm } from '../../common/validate';
import {
    interaction_choose_metadata_get_output_type,
    interaction_choose_metadata_get_used_component,
    InteractionChooseMetadata,
} from './choose';
import {
    interaction_choose_form_metadata_get_output_type,
    interaction_choose_form_metadata_get_used_component,
    InteractionChooseFormMetadata,
} from './choose_form';
import {
    interaction_choose_full_metadata_get_output_type,
    interaction_choose_full_metadata_get_used_component,
    InteractionChooseFullMetadata,
} from './choose_full';
import {
    interaction_choose_tip_metadata_get_output_type,
    interaction_choose_tip_metadata_get_used_component,
    InteractionChooseTipMetadata,
} from './choose_tip';

export interface ComponentInteraction {
    id: ComponentId;
    inlets?: Endpoint[];
    metadata: InteractionMetadata;
}

export interface InteractionMetadata {
    name?: string;
    metadata: InteractionInnerMetadata;
}

export type InteractionInnerMetadata =
    | { choose: InteractionChooseMetadata }
    | { choose_form: InteractionChooseFormMetadata }
    | { choose_tip: InteractionChooseTipMetadata }
    | { choose_full: InteractionChooseFullMetadata };

export const match_interaction_inner_metadata = <T>(
    self: InteractionInnerMetadata,
    {
        choose,
        choose_form,
        choose_tip,
        choose_full,
    }: {
        choose: (choose: InteractionChooseMetadata) => T;
        choose_form: (choose_form: InteractionChooseFormMetadata) => T;
        choose_tip: (choose_tip: InteractionChooseTipMetadata) => T;
        choose_full: (choose_full: InteractionChooseFullMetadata) => T;
    },
): T => {
    if ('choose' in self) return choose(self.choose);
    if ('choose_form' in self) return choose_form(self.choose_form);
    if ('choose_tip' in self) return choose_tip(self.choose_tip);
    if ('choose_full' in self) return choose_full(self.choose_full);
    throw new Error('Invalid interaction inner metadata');
};

export const component_interaction_get_output_type = (self: ComponentInteraction): LinkType => {
    return match_interaction_inner_metadata<LinkType>(self.metadata.metadata, {
        choose: (choose) => interaction_choose_metadata_get_output_type(choose),
        choose_form: (choose_form) => interaction_choose_form_metadata_get_output_type(choose_form),
        choose_tip: (choose_tip) => interaction_choose_tip_metadata_get_output_type(choose_tip),
        choose_full: (choose_full) => interaction_choose_full_metadata_get_output_type(choose_full),
    });
};

export const component_interaction_get_used_component = (self: ComponentInteraction): ComponentId[] => {
    return match_interaction_inner_metadata(self.metadata.metadata, {
        choose: (choose) => interaction_choose_metadata_get_used_component(choose),
        choose_form: (choose_form) => interaction_choose_form_metadata_get_used_component(choose_form),
        choose_tip: (choose_tip) => interaction_choose_tip_metadata_get_used_component(choose_tip),
        choose_full: (choose_full) => interaction_choose_full_metadata_get_used_component(choose_full),
    });
};

export const component_interaction_get_validate_form = (self: ComponentInteraction): ValidateForm | undefined => {
    return match_interaction_inner_metadata(self.metadata.metadata, {
        choose: () => undefined,
        choose_form: (choose_form) => choose_form.validate,
        choose_tip: () => undefined,
        choose_full: (choose_full) => choose_full.form?.validate,
    });
};
