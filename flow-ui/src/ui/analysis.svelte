<script lang="ts">
  import type { ChartDataset } from "chart.js";
  import Card from "../components/card.svelte";
  import { Doughnut } from "../components/charts";
  import { projectService } from "../services";

  let chartData = projectsData();

  async function projectsData(): Promise<ChartDataset<"doughnut">[]> {
    const projects = await projectService.get_all();
    const data = projects.map(({ tasks }) => tasks.length);
    return [
      {
        data,
        backgroundColor: ["red", "yellow"],
        borderWidth: 1,
      },
    ];
  }
</script>

<section class="main-info flex flexcol mt-4">
  <div class="w-2/5 h-full">
    <Card title="Projects">
      {#await chartData}
        <p>loading...</p>
      {:then data}
        <Doughnut datasets={data} options={{}} />
      {:catch e}
        <p>{e.message}</p>
      {/await}
    </Card>
  </div>
  <div class="info-cards w-3/5 flex flex-row flex-wrap">
    <Card
      title="Task Total"
      className="aspect-square"
      sectionStyle="h-full flex items-center justify-center"
    >
      <h2 class="font-bold text-4xl">4</h2>
    </Card>
    <Card
      title="Todo"
      className="aspect-square"
      sectionStyle="h-full flex items-center justify-center"
    >
      <h2 class="font-bold text-4xl">4</h2>
    </Card>
    <Card
      title="In Progress"
      className="aspect-square"
      sectionStyle="h-full flex items-center justify-center"
    >
      <h2 class="font-bold text-4xl">4</h2>
    </Card>
    <Card
      title="Done"
      className="aspect-square"
      sectionStyle="h-full flex items-center justify-center"
    >
      <h2 class="font-bold text-4xl">4</h2>
    </Card>
  </div>
</section>

<style>
  .main-info {
    gap: 1rem;
    max-height: 370px;
    overflow: hidden;
  }

  .main-info > div:first-child :global(> div) {
    height: 100%;
  }

  .main-info > div {
    height: inherit;
  }

  .info-cards {
    gap: 1rem;
  }

  .info-cards :global(> *) {
    width: calc(50% - 0.5rem);
    height: calc(50% - 0.5rem);
  }
</style>
