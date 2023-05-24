<script lang="ts">
  export let title: string;
  export let className = "";

  export let headerStyle = "";
  export let sectionStyle = "";
</script>

<div
  class="flex flex-col justify-between card bg-soft p-4 rounded-lg {className}"
>
  <header class="flex flex-row {headerStyle}">
    <h3 class="font-semibold text-lg">{title}</h3>
  </header>
  <section class="flex justify-center {sectionStyle}">
    <slot />
  </section>
</div>

<style>
  div {
    display: grid !important;
    grid-template-rows: 8% 92%;
    grid-template-columns: 1fr;
  }
</style>
