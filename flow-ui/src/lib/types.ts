export const enum TaskStatus {
  NotStarted = "NoStarted",
  InProgress = "InProgress",
  Stop = "Stop",
  Done = "Complete",
}

export interface Project {
  id: number;
  path: string;
  tasks: Task[];
  name: string;
}

export interface Task {
  description: string;
  id: number;
  status: TaskStatus;
  tempId: number;
}
