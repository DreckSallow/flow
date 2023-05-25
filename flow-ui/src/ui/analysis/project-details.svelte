<script lang="ts">
  import Card from "../../components/card.svelte";
  import Percentage from "../../components/percentage.svelte";
  import { TaskStatus, type Project } from "../../lib/types";

  export let projects: Project[];

  let statusColors = {
    todo: "#FFD54F",
    inProgress: "#79CFFC",
    done: "#81C784",
    stop: "#FF8A80",
  };

  function getProjectData(project: Project) {
    console.log(project.tasks);
    const grouped: { [key in keyof typeof statusColors] } = {
      todo: project.tasks.filter(
        ({ status }) => status === TaskStatus.NotStarted
      ).length,
      inProgress: project.tasks.filter(
        ({ status }) => status === TaskStatus.InProgress
      ).length,
      done: project.tasks.filter(({ status }) => status === TaskStatus.Done)
        .length,
      stop: project.tasks.filter(({ status }) => status === TaskStatus.Stop)
        .length,
    };
    return Object.entries(grouped).map(([k, count]) => {
      return {
        color: statusColors[k],
        label: "DONE",
        data: count,
      };
    });
  }
</script>

<div class="main-info flex mt-4 flex-row gap-4">
  {#each projects as project}
    <Card
      title={project.name}
      sectionStyle="flex flex-col gap-4"
      className="w-60 h-24"
    >
      <!-- <h4 class="font-medium text-soft text-sm mt-4">{project.path}</h4> -->
      <Percentage
        dataPercentage={getProjectData(project)}
        class="w-4/5 bg-strong"
      />
    </Card>
  {/each}
</div>
