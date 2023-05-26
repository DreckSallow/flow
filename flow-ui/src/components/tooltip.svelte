<script lang="ts">
  import { tooltipStore } from "../lib/store/tooltip";
  let tooltip: HTMLElement;

  let cursor = {
    x: 0,
    y: 0,
  };

  function setCursor() {
    setTimeout(() => {
      let left = (tooltip?.offsetWidth ?? 0) / 2;
      cursor = {
        x: ($tooltipStore?.cursor.x ?? left) - left,
        y: $tooltipStore?.cursor.y || 0,
      };
    }, 100);
  }

  $: if ($tooltipStore) {
    setCursor();
  } else {
    cursor = { x: 0, y: 0 };
  }
</script>

<div
  id="tooltip"
  bind:this={tooltip}
  style:left="{cursor.x}px"
  style:top="{cursor.y}px"
  class="absolute bg-strong text-strong p-2 rounded-md text-sm"
  class:watch={$tooltipStore}
>
  {$tooltipStore?.text ?? ""}
</div>

<style>
  div {
    z-index: -2;
  }

  .watch {
    z-index: 999;
  }
</style>
