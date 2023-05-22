<slot />

<style>
  :global(:root) {
    --alpha: 1;
  }

  :global(.light-theme) {
    --bg-strong: 233, 67%, 98%;
    --bg-soft: 44, 0%, 100%;
  }

  :global(.dark-theme) {
    --bg-back: 253, 5%, 26%;
    --bg-soft: 217, 20%, 98%;
  }
</style>
