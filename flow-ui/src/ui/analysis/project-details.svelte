<script lang="ts">
  import Card from "../../components/card.svelte";
  import { CloseIcon } from "../../components/icons";
  import Percentage from "../../components/percentage.svelte";
  import { clickOutside } from "../../lib/events";
  import { TaskStatus, type Project } from "../../lib/types";

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
  let modalData: Project | null = null;
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
        emptyLabel="No have tasks"
        dataPercentage={getProjectData(project)}
        class="w-4/5 bg-strong cursor-pointer"
        on:click={() => {
          modalData = project;
        }}
      />
    </Card>
  {/each}
</div>
{#if modalData}
  <div
    style="--alpha:0.8;"
    class="modal fixed top-0 left-0 right-0 bottom-0 grid place-content-center bg-strong"
  >
    <div
      style="--alpha:1;"
      class="flex flex-col bg-soft p-4 text-strong rounded-lg relative"
      use:clickOutside
      on:outclick={() => (modalData = null)}
    >
      <button
        style="--alpha:0.5"
        class="modal-close absolute top-1 right-1 bg-soft rounded-md p-0.5"
        on:keydown
        on:click={() => (modalData = null)}
      >
        <CloseIcon class="w-4 h-4" />
      </button>
      <h3 class="mt-3">{modalData.name}</h3>
      <div class="bg-soft rounded-lg overflow-hidden border border-slate-400">
        <table class="table-aut">
          <thead>
            <!-- <tr><th colspan="3">TASKS DATA</th></tr> -->
            <tr>
              <th class="border border-slate-300">Id</th>
              <th class="border border-slate-300">Description</th>
              <th class="border border-slate-300">Status</th>
            </tr>
          </thead>
          <tbody>
            {#each modalData.tasks as task}
              <tr>
                <td class="border border-slate-300">{task.id}</td>
                <td class="border border-slate-300">{task.description}</td>
                <td class="border border-slate-300">{task.status}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal .modal-close:hover {
    --alpha: 1 !important;
  }

  .modal table th,
  td {
    padding: 1rem;
  }
</style>
