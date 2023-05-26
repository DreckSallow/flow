<script lang="ts">
  import { TaskStatus, type Project } from "../../lib/types";
  import Details from "./details.svelte";

  export let projects: Project[];

  let statusColors = {
    todo: "#FFD54F",
    inProgress: "#79CFFC",
    done: "#81C784",
    stop: "#FF8A80",
  };

  function getProjectData(project: Project) {
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
        label: k,
        data: count,
      };
    });
  }
</script>

<div class="main-info flex mt-4 flex-col gap-4">
  {#each projects as project}
    <Details {project} projectTaskData={getProjectData(project)} />
  {/each}
</div>
