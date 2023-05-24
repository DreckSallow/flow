<script lang="ts">
  import Card from "../../components/card.svelte";
  import { TaskStatus } from "../../lib/types";
  import { taskService } from "../../services";

  type TaskInfo = { [key in TaskStatus]: number } & { Total: number };

  let taskInfo: Promise<TaskInfo> = getTaskInfo();

  async function getTaskInfo(): Promise<TaskInfo> {
    let tasks = await taskService.get_all();
    return {
      Total: tasks.length,
      [TaskStatus.Stop]: tasks.reduce(
        (acc, { status }) => (status === TaskStatus.Stop ? acc + 1 : acc),
        0
      ),
      [TaskStatus.InProgress]: tasks.reduce(
        (acc, { status }) => (status === TaskStatus.InProgress ? acc + 1 : acc),
        0
      ),
      [TaskStatus.Done]: tasks.reduce(
        (acc, { status }) => (status === TaskStatus.Done ? acc + 1 : acc),
        0
      ),
      [TaskStatus.NotStarted]: tasks.reduce(
        (acc, { status }) => (status === TaskStatus.NotStarted ? acc + 1 : acc),
        0
      ),
    };
  }
</script>

<div class="info-cards w-3/5 flex flex-row flex-wrap">
  <Card
    title="Task Total"
    className="aspect-square"
    sectionStyle="h-full flex items-center justify-center"
  >
    {#await taskInfo}
      <p>Loading...</p>
    {:then { Total }}
      <h2 class="font-bold text-4xl text-soft">{Total}</h2>
    {:catch}
      <p>Error</p>
    {/await}
  </Card>
  <Card
    title="Todo"
    className="aspect-square"
    sectionStyle="h-full flex items-center justify-center"
  >
    {#await taskInfo}
      <p>Loading...</p>
    {:then { NoStarted }}
      <h2 class="font-bold text-4xl text-soft">{NoStarted}</h2>
    {:catch}
      <p>Error</p>
    {/await}
  </Card>
  <Card
    title="In Progress"
    className="aspect-square"
    sectionStyle="h-full flex items-center justify-center"
  >
    {#await taskInfo}
      <p>Loading...</p>
    {:then { InProgress }}
      <h2 class="font-bold text-4xl text-soft">{InProgress}</h2>
    {:catch}
      <p>Error</p>
    {/await}
  </Card>
  <Card
    title="Done"
    className="aspect-square"
    sectionStyle="h-full flex items-center justify-center"
  >
    {#await taskInfo}
      <p>Loading...</p>
    {:then { Complete }}
      <h2 class="font-bold text-4xl text-soft">{Complete}</h2>
    {:catch e}
      <p>Error {e}</p>
    {/await}
  </Card>
</div>

<style>
  .info-cards {
    gap: 1rem;
  }

  .info-cards :global(> *) {
    width: calc(50% - 0.5rem);
    height: calc(50% - 0.5rem);
  }
</style>
