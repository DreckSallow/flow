enum TaskStatus {
  NotStarted,
}

export interface Project {
  id: number;
  path: string;
  tasks: Task[];
}

export interface Task {
  description: string;
  id: number;
  status: TaskStatus;
  tempId: number;
}
