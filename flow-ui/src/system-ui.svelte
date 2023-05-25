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
    --text-strong: 0, 0%, 100%;
    --text-soft: 246, 36%, 71%;
    --bg-strong: 232, 34%, 17%;
    --bg-soft: 231, 32%, 22%;
  }
</style>
