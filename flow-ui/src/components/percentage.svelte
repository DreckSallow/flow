<script lang="ts">
  type PercentageData = {
    label: string;
    data: number;
    color: string;
  };

  export let dataPercentage: PercentageData[];

  $: total = dataPercentage.reduce((acc, { data }) => acc + data, 0);
  $: cleanedData = (() => {
    let left = 0;
    return dataPercentage.map(({ data, ...rest }) => {
      let percentage = (data * 100) / total;

      let obj = {
        ...rest,
        percentage,
        left,
      };
      left += percentage;
      return obj;
    });
  })();
</script>

<div class="h-5 rounded-full overflow-hidden {$$props.class}">
  {#each cleanedData as data}
    <span
      class="percentage h-full inline-block"
      style="background-color: {data.color};  width: {data.percentage}%;left: {data.left}%"
    />
  {/each}
</div>
