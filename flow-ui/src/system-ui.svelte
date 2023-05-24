<slot />

<style>
  :global(:root) {
    --alpha: 1;
  }

  :global(.light-theme) {
    --text-strong: 215, 5%, 18%;
    --text-soft: 232, 9%, 52%;
    --bg-strong: 233, 67%, 98%;
    --bg-soft: 44, 0%, 100%;
  }

  :global(.dark-theme) {
    --text-strong: 215°, 5%, 18%;
    --bg-back: 253, 5%, 26%;
    --bg-soft: 217, 20%, 98%;
  }
</style>
