<script context="module" lang="ts">
  let currentOpen: Project["path"];
  let closeOther = () => {};
</script>

<script lang="ts">
  import Percentage from "../../components/percentage.svelte";
  import type { Project } from "../../lib/types";

  export let project: Project;
  export let projectTaskData: { color: string; data: number; label: string }[];
  export let open: boolean = false;

  function toggleDisplay() {
    if (currentOpen && !(currentOpen == project.path)) {
      closeOther();
    }
    open = !open;
    currentOpen = project.path;
    closeOther = () => (open = false);
  }
</script>

<article>
  <div
    on:click={toggleDisplay}
    on:keydown
    class:rounded-b-lg={!open}
    class="flex flex-row gap-4 items-center justify-start bg-soft rounded-t-lg p-6 cursor-pointer"
  >
    <span class="text-strong text-lg">{project.name}</span>
    <!-- <h4 class="font-medium text-soft text-sm mt-4">{project.path}</h4> -->
    <Percentage
      on:click={(e) => {
        e.stopPropagation();
      }}
      emptyLabel="No have tasks"
      dataPercentage={projectTaskData}
      class="percentage-detail bg-strong cursor-pointer h-6"
    />
  </div>
  {#if open}
    <div style="--alpha:0.6;" class="bg-soft p-3 rounded-b-lg">
      <div
        style="--alpha:1;"
        class="table-container text-strong rounded-lg overflow-hidden"
      >
        {#if project.tasks.length > 0}
          <table class="table-auto w-full text-center" style="--alpha:0.6;">
            <thead class="bg-soft" style="--alpha:1;">
              <tr class="text-soft">
                <th>N°</th>
                <th>Id</th>
                <th>Description</th>
                <th>Status</th>
              </tr>
            </thead>
            <tbody>
              {#each project.tasks as task}
                <tr>
                  <td>{task.tempId}</td>
                  <td>{task.id}</td>
                  <td>{task.description}</td>
                  <td>{task.status}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        {:else}
          <p class="text-lg text-soft text-center p-6">No have Tasks</p>
        {/if}
      </div>
    </div>
  {/if}
</article>

<style>
  article span {
    width: 150px;
    min-width: 50px;
  }

  article :global(.percentage-detail) {
    width: 300px;
    min-width: 100px;
  }
  table td,
  th {
    padding: 2em 4em;
  }

  div.table-container {
    overflow: auto;
    max-height: 500px;
  }

  div.table-container table thead {
    position: sticky;
    z-index: 99;
    top: 0;
  }
</style>
