# todosh

A lightweight CLI to do list manager.

<p align="center">
  <img src="https://i.imgur.com/w1Ebztp.png">
</p>

Open tasks stored in local `task.todo` file, with completed tasks stored in `completed.todo`.  
<br/>

I made this for personal use because I find myself in Terminal so often that it made sense to keep track of tasks in it, rather than having to tab out to another program to see them. It also means that tasks are stored alongside projects. It was also a good opportunity to have a go at writing some Rust.  
<br/> 
This is by no means well written, in fact it is probably terribly written—but it works for my needs.

## Installation

```shell
brew tap jonlinkens/tap
brew install todosh
```

## Commands

- **Creating a list**  
  ```shell
  todosh create
  ```
- **Listing tasks**  
  Shows all open tasks and the 5 most recently completed tasks.
  ```shell
  todosh list
  ```
- **Adding a task**
  ```shell
  todosh create "task here"
  ```
- **Finishing a task**  
  ```shell
  todosh finish <task_number>
  ```
- **Deleting a list**
  ```shell
  todosh delete
  ```
