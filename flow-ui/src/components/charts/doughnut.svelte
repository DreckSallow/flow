<script lang="ts">
  import { onMount } from "svelte";
  import Chart, {
    type ChartDataset,
    type CoreChartOptions,
  } from "chart.js/auto";
  export let datasets: ChartDataset<"doughnut">[] = [];
  export let options: Partial<CoreChartOptions<"doughnut">>;
  export let labels: string[] = [];

  let ctxCanvas: HTMLCanvasElement | null;

  onMount(() => {
    if (!ctxCanvas) {
      throw new Error("Error reading the canvas");
    }

    const ctx = ctxCanvas.getContext("2d");
    new Chart(ctx, {
      type: "doughnut",
      data: {
        labels,
        datasets,
      },
      options: {
        elements: {
          arc: {
            borderWidth: 0,
          },
        },
        ...options,
      },
    });
  });
</script>

<canvas bind:this={ctxCanvas} />
