<script lang="ts">
  import { onMount } from "svelte";
  import SystemUi from "./system-ui.svelte";
  import Analysis from "./ui/analysis/analysis.svelte";
  import Nav from "./ui/nav.svelte";
  import { getTheme } from "./lib/store/theme";

  onMount(() => {
    getTheme();
  });
</script>

<SystemUi>
  <Nav />
  <section>
    <h2 class="text-xl font-semibold text-strong">Analytics Overview</h2>
    <Analysis />
  </section>
</SystemUi>
