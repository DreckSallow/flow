<script lang="ts">
  import Card from "../../components/card.svelte";
  import { Doughnut } from "../../components/charts";
  import TaskInfo from "./task-info.svelte";
  import type { Project } from "../../lib/types";

  export let projects: Project[];

  $: chartData = {
    dataset: [
      {
        label: "Tasks",
        data: projects.map(({ tasks }) => tasks.length),
      },
    ],
    labels: projects.map(({ name }) => name),
  };
  $: console.log(chartData);
</script>

<section class="main-info flex mt-4 flex-row">
  <div class="w-2/5 h-full">
    <Card title="Tasks overview">
      <Doughnut
        datasets={chartData.dataset}
        options={{}}
        labels={chartData.labels}
      />
    </Card>
  </div>
  <TaskInfo />
</section>

<style>
  .main-info {
    gap: 1rem;
    max-height: 370px;
    padding-bottom: 1em;
    overflow: hidden;
  }

  .main-info > div:first-child :global(> div) {
    height: 100%;
  }

  .main-info > div {
    height: inherit;
  }
</style>
