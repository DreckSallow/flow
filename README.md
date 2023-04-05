# Flow

Flow is command line, that help you to manage much better your projects and tasks.
You can create a project and create tasks related to this project.

One more thing, if you love this project, add a star please :).

# Installation

The releases are [here](https://github.com/DreckSallow/flow/releases), where you can download the binary
and run.

Using git:

```
git clone https://github.com/DreckSallow/flow.git
cd flow
cargo build --release
```

Using cargo:

```
cargo install flow-cli
```

# Overview

To see a description of each command, run: `flow <command> --help`

The app is organized into the following subcommands:

## project

Executed as `flow project`, display the current project path.

If you want to add a project:

`flow project <PATH>`

And if you want to create a folder, you can run:

`flow project <PATH> --new`

### list

List all projects

`flow project list`

### switch

Set other project as current project using the project `<ID>`.

`flow project switch <ID>`

### rm

Remove a project using the project `<ID>`

`flow project rm <ID>`

### use

Flow reads the path where it is called, and then set this path (if exists in the saved projects) as the current project.

`flow project use`

## task

To create a new task: `flow task -d <DESCRIPTION>`

### list

List all tasks related to current project.

`flow task list`

To list more info: `flow task list --expand`.

To display with colors: `flow task list --color`.

To order by: `flow task list --order-by <'number'|'desc'>`.

### rm

Remove a task using the task `<N-ID>`

`flow task rm <N-ID>`

### start

Mark a task as "In Progress".

`flow task start <N-ID>`

> ⚠️ You cannot have two tasks with the same "In Progress" status.

### Stop

Mark a task as "Stop".

`flow task stop <N-ID>`

### done

Mark tasks as "Complete". This command accepts multiple `<N-IDS>`

`flow task done <N-IDS>`

### reset

Restore the status tasks as "Not Started". This command accepts multiple `<N-IDS>`

`flow task reset <N-IDS>`

# Documentation

The documentation is under development.

# Alternatives

- [TaskWarrior](https://github.com/GothenburgBitFactory/taskwarrior) is a command line task list.
- [TaskBook](https://github.com/klaudiosinani/taskbook)

# License

Flow is licensed under the [MIT license](LICENSE).
