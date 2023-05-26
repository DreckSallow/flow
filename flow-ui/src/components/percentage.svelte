<script lang="ts">
  import { displayTooltip } from "../lib/store/tooltip";
  import { getExactPosition } from "../lib/utils";

  type PercentageData = {
    label: string;
    data: number;
    color: string;
  };

  export let dataPercentage: PercentageData[];

  export let emptyLabel: string;

  $: total = dataPercentage.reduce((acc, { data }) => acc + data, 0);
  $: cleanedData = (() => {
    let left = 0;
    return dataPercentage.map(({ data, ...rest }) => {
      let percentage = (data * 100) / total;

      let obj = {
        ...rest,
        percentage,
        count: data,
        left,
      };
      left += percentage;
      return obj;
    });
  })();
</script>

<div
  class="h-5 rounded-full overflow-hidden relative {$$props.class}"
  on:click
  on:keydown
>
  {#each cleanedData as data}
    <span
      on:mouseenter={(e) => {
        let { width } = e.currentTarget.getBoundingClientRect();
        const { left, top } = getExactPosition(e.currentTarget);

        displayTooltip({
          text: data.label + ": " + data.count,
          cursor: {
            x: left + width / 2,
            y: top - 5,
          },
        });
      }}
      on:mouseleave={(e) => {
        displayTooltip(null);
      }}
      class="h-full inline-block"
      style="background-color: {data.color};  width: {data.percentage}%;left: {data.left}%"
    />
  {/each}
  {#if cleanedData.length > 0 || cleanedData.every(({ count }) => count == 0)}
    <span
      class="w-full h-full inline-block absolute"
      on:mouseenter={(e) => {
        let { width } = e.currentTarget.getBoundingClientRect();
        const { left, top } = getExactPosition(e.currentTarget);
        displayTooltip({
          text: emptyLabel,
          cursor: {
            x: left + width / 2,
            y: top - 4,
          },
        });
      }}
      on:mouseleave={(e) => {
        displayTooltip(null);
      }}
    />
  {/if}
</div>
