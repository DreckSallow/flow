<script lang="ts">
  import { tooltipStore } from "../lib/store/tooltip";
  let tooltip: HTMLElement;

  let cursor = {
    x: 0,
    y: 0,
  };

  function setCursor() {
    setTimeout(() => {
      const left = (tooltip?.offsetWidth ?? 0) / 2;
      const right = tooltip?.offsetHeight ?? 0;
      cursor = {
        x: ($tooltipStore?.cursor.x ?? left) - left,
        y: ($tooltipStore?.cursor.y ?? right) - right,
      };
    }, 100);
  }

  $: if ($tooltipStore) {
    setCursor();
  } else {
    cursor = { x: 0, y: 0 };
  }
</script>

<!-- <span
  class="visible w-3 h-3 bg-red-300 absolute"
  style:left="{$tooltipStore?.cursor.x ?? 0}px"
  style:top="{$tooltipStore?.cursor.y ?? 0}px"
/> -->

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
  /* span {
    z-index: 9999;
  } */
  div {
    z-index: -2;
  }

  .watch {
    z-index: 999;
  }
</style>
