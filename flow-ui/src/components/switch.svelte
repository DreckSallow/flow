<script lang="ts">
  export let checked = false;
</script>

<button
  class="switch-button rounded-full border relative h-8 w-14 bg-strong overflow-hidden"
  role="switch"
  aria-checked={checked}
  on:click={() => (checked = !checked)}
>
  <span
    class="rounded-full absolute w-1/2 h-full overflow-hidden bg-soft flex items-center justify-center"
    class:switch={checked}
  >
    <slot {checked} />
  </span>
</button>

<style>
  span.switch {
    transform: translateX(calc(100% - 6px));
  }

  span {
    left: 0;
    top: 0;
    height: calc(100% - 6px);
    margin: 3px;
    transition: transform 350ms ease;
  }
</style>
