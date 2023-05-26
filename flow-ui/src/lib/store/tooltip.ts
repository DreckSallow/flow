import { writable } from "svelte/store";

export type TooltipData = {
  text: string;
  cursor: {
    x: number;
    y: number;
  };
};

export type TooltipStoreType = TooltipData | null;

export const tooltipStore = writable<TooltipStoreType>(null);

export const displayTooltip = (data: TooltipStoreType) => {
  tooltipStore.set(data);
};
