<script lang="ts">
  export let title: string;
  export let className = "";

  export let headerStyle = "";
  export let sectionStyle = "";
</script>

<div
  class="flex flex-col justify-between card bg-soft p-4 rounded-lg card-shadow {className}"
>
  <header class="flex flex-row {headerStyle} text-strong">
    <h3 class="font-semibold text-lg">{title}</h3>
  </header>
  <section
    class="flex justify-center items-center min-w-0 w-full h-full {sectionStyle}"
  >
    <slot />
  </section>
</div>
