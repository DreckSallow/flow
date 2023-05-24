import type { Project, Task } from "../lib/types";

export class ProjectService {
  private endpoint = "/api/projects";

  async get_all(): Promise<Array<Project>> {
    const res = await fetch(this.endpoint);
    const dataRes = (await res.json()) as Array<any>;
    return dataRes.reduce<Array<Project>>((acc, { id, path, tasks }) => {
      const cleanTasks = tasks.reduce(
        (acc: Array<Task>, { temp_id, ...all }) => {
          return acc.concat({
            tempId: temp_id,
            ...all,
          } as Task);
        },
        [] as Array<Task>
      );

      return acc.concat({
        id,
        path,
        tasks: cleanTasks,
      });
    }, []);
  }
}
