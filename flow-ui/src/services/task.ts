import type { Task } from "../lib/types";

export class TaskService {
  private endpoint = "/api/tasks";

  async get_all(): Promise<Array<Task>> {
    const res = await fetch(this.endpoint);
    const dataRes = (await res.json()) as Array<any>;

    return dataRes.reduce<Array<Task>>((acc, { temp_id, ...all }) => {
      return acc.concat({
        tempId: temp_id,
        ...all,
      });
    }, [] as Array<Task>);
  }
}
