<script lang="ts">
  import SystemUi from "./system-ui.svelte";
  import Analysis from "./ui/analysis/analysis.svelte";
  import Nav from "./ui/nav.svelte";
</script>

<SystemUi>
  <Nav />
  <section>
    <h2 class="text-xl font-semibold">Analytics Overview</h2>
    <Analysis />
  </section>
</SystemUi>
