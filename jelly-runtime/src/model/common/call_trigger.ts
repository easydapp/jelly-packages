import { ComponentId } from './identity';
import { input_value_get_used_component, InputValue } from './refer';

export interface CallTriggerLoading {
    alive?: number;
}

export interface CallTriggerClock {
    sleep: number;
    loading?: boolean;
}

export interface CallTriggerClick {
    text?: InputValue;
}

export type ComponentCallTrigger =
    | { loading: CallTriggerLoading }
    | { clock: CallTriggerClock }
    | { click: CallTriggerClick };

export const match_component_call_trigger = <T>(
    self: ComponentCallTrigger,
    {
        loading,
        clock,
        click,
    }: {
        loading: (loading: { alive?: number }) => T;
        clock: (clock: { sleep: number; loading?: boolean }) => T;
        click: (click: { text?: InputValue }) => T;
    },
): T => {
    if ('loading' in self) return loading(self.loading);
    if ('clock' in self) return clock(self.clock);
    if ('click' in self) return click(self.click);
    throw new Error('invalid call trigger');
};

export const call_trigger_get_used_component = (self: ComponentCallTrigger): ComponentId[] => {
    const used: ComponentId[] = [];
    match_component_call_trigger(self, {
        loading: () => {
            /* do nothing */
        },
        clock: () => {
            /* do nothing */
        },
        click: (click) => {
            if (click.text) used.push(...input_value_get_used_component(click.text));
        },
    });
    return used;
};
