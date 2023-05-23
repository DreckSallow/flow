<script lang="ts">
  import { onMount } from "svelte";
  import Chart from "chart.js/auto";

  let ctxCanvas: HTMLCanvasElement | null;

  onMount(() => {
    if (!ctxCanvas) {
      throw new Error("Error reading the canvas");
    }

    const ctx = ctxCanvas.getContext("2d");
    new Chart(ctx, {
      type: "doughnut",
      data: {
        datasets: [
          {
            // label: "My First Dataset",
            data: [300, 50, 100],
            backgroundColor: ["#7000e1", "#fc8800", "#00b0e8"],
            // hoverOffset: 4,
            borderWidth: 0,
          },
        ],
      },
      options: {
        responsive: true,
        spacing: 0,
      },
    });
  });
</script>

<canvas bind:this={ctxCanvas} />
