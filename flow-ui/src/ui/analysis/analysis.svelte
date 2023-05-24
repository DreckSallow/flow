<script lang="ts">
  import type { ChartDataset } from "chart.js";
  import Card from "../../components/card.svelte";
  import { Doughnut } from "../../components/charts";
  import { projectService } from "../../services";
  import TaskInfo from "./task-info.svelte";

  type DataChart = {
    dataset: ChartDataset<"doughnut">[];
    labels?: string[];
  };

  let chartData: Promise<DataChart> = projectsData();

  async function projectsData(): Promise<DataChart> {
    const projects = await projectService.get_all();
    return {
      dataset: [
        {
          label: "Tasks",
          data: projects.map(({ tasks }) => tasks.length),
        },
      ],
      labels: projects.map(({ name }) => name),
    };
  }
</script>

<section class="main-info flex flexcol mt-4">
  <div class="w-2/5 h-full">
    <Card title="Projects">
      {#await chartData}
        <p>loading...</p>
      {:then data}
        <Doughnut datasets={data.dataset} options={{}} labels={data.labels} />
      {:catch e}
        <p class="text-red-400">Error getting the data</p>
      {/await}
    </Card>
  </div>
  <TaskInfo />
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
</style>
