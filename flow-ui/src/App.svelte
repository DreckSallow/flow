<script lang="ts">
  import { onMount } from "svelte";
  import SystemUi from "./system-ui.svelte";
  import Analysis from "./ui/analysis/analysis.svelte";
  import Nav from "./ui/nav.svelte";
  import { getTheme } from "./lib/store/theme";
  import ProjectDetails from "./ui/analysis/project-details.svelte";
  import { projectService } from "./services";
  import Tooltip from "./components/tooltip.svelte";

  onMount(() => {
    getTheme();
  });

  let projects = projectService.get_all();
</script>

<SystemUi>
  <Nav />
  <section>
    <h2 class="text-xl font-semibold text-strong">Analytics Overview</h2>
    {#await projects}
      <p class="text-soft">Loading...</p>
    {:then data}
      <Analysis projects={data} />
    {:catch e}
      <p class="text-red-400">Error getting the projects</p>
    {/await}
  </section>
  <section>
    <h2 class="text-xl font-semibold text-strong">Project Details</h2>
    {#await projects}
      <p class="text-soft">Loading...</p>
    {:then data}
      <ProjectDetails projects={data} />
    {:catch e}
      <p class="text-red-400">Error getting the projects</p>
    {/await}
  </section>
</SystemUi>
<Tooltip />
