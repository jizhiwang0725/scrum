# 🚀 Agile Scrum Project
Welcome to our project management board! This workspace is dedicated to tracking our Agile software development lifecycle, managing Sprints (iterations), and coordinating team efforts throughout the development process.

# 📌 Project Overview
This is the single source of truth for our product backlog, Sprint planning, and daily development tasks. We strictly adhere to Scrum methodologies to ensure iterative delivery and continuous improvement.

# 🌍 Language Policy 
**Project Management** (Issues/Kanban) is conducted in **Chinese** :cn:.  
**Version Control** (Commits/Branches) must be in **English** :gb:.

# :warning: Team Workflow Rule 
***Main Branch Protection***  
Direct pushes to the `main` branch are strictly prohibited. All code changes must be made on separate feature branches and merged into the main branch via a Pull Request (PR) after a Code Review.
  
***Branch Naming Convention***  
`<type>/<short-description>`
  - `feat/user-login`


# Pull Request Standards
***Naming Convention***  
- Follow the same format used in one-line commits  
`<type>(<scope>): <subject>`  

***Pull Request Description***
- Require developers to fill out the **checklist in the PR description**, expanding on the "why this change was made" concept you currently suggest for complex commits.

# Issues 
| Name | Function |
| --- | --- |
| 🌟 用户故事 (User Story) | 提交一个新的产品功能或业务需求 |
| 🔬 技术探路 (Spike) | 有时间限制的技术调研、选型对比或概念验证 (PoC) | 
| 💻 开发子任务 (Dev Task) | 为实现某个 User Story 而拆解的具体功能代码开发 |
| ⚙️ 技术任务 (Technical Task) | 基础设施搭建、环境配置、Git演练或代码重构 |
| 🐛 缺陷报告 (Bug Report) | 报告主分支代码或产品中的错误行为 |

# Committing Message 
## One-line 
***Structure***  
`<type>(<scope>): <subject>`

| Type | Function |
| --- | --- |
| `feat` | - A new feature that benefits the users |
| `fix` | - A bug fix | 
| `docs` | - Documentation only changes (like README.md) |
| `style` | - Formatting changes that **do not** affect the code's meaning<br> (white-space, formatting) | 
| `refactor` | - A code change that neither fixes a bug nor adds a feature,<br> just **optimizes structure**
| `test` | - Adding missing tests or correcting existing tests | 
| `chore` | - Changes to build process or auxiliary tools for developers only (e.g. configuring `gitignore`)

***Scope***
- The scope indicate **which module you modified** (e.g., (ui) or (database); this can be omitted initially).

***Subject***
- Always **start with a lowercase**
- **Concise and clear**, summarizing your changes in a single sentence 
- **Always start the subject with a present verb**
- If the code for a particular commit is extremely complex, you can **leave a blank line after the subject** and write a multiline Body to explain in detail "why this changes was made," though this is usually better suited for the **Pull Request description on GitHub**

## Multi-line
**Dominated by the Primary Purpose**. For those secondary changes, use the **multi-line Body** provided.  

```bash
feat(backend): add user login module

- feat: support basic email and password validation
- fix: resolve layout misplacement in navigation bar
- docs: update README to add environment dependency instructions
```

# Committing Criteria 
***Atomic Commits***  
Specify exact files: `git add login.py` then write a `feat` commit; subsequently, execute `git add README.md`then write a `docs` commit.  

# Integration & Conflict Resolution
## Defensive committing 
You've been writing UI code on your `feat-ui` branch for three days and are ready to push it to the cloud. However, you know perfectly well that other team members must have merged other code into the `main` branch during these three days. To avoid throwing a merge conflict onto GitHub for others to stress over, you decide to **integrate it locally first**  


## Instructions
1. Ensure your current code is already tested and committed   
2. **Stay on your** `feat-ui` **branch**, and directly run `git pull origin main`
3. At this point, a bunch of conflicts will likely pop up. Don't panic. Open VS Code, find the files marked in red, click the visual **"Accept Both Changes"** (or which ever option you deem correct), and save the files.
4. Commit the resolution
  `git commit -m "chore: resolve merge conflicts with main"`
5. Push your integrated code to the cloud: `git push`

# Workflow Closing 
***Local Cleanup After PR***  
Once your PR has been reviewed and merged on GitHub, return to your local terminal and execute the following commands to keep your repository clean 

``` bash
# 1. Switch to the main branch and sync the latest team progress
git switch main
git pull

# 2. Clean up local tracking branches that have been deleted on the remote
git fetch --prune

# 3. Delete your local feature branch
git branch -d <your-branch-name>
```

# 📋 Project Views
To maintain a clear and focused workflow, this board uses filters and custom fields to divide the workspace into the following dedicated views:

- **_Current Sprint_**  
The active development board. It displays only the user stories and specific tasks scheduled for the current iteration, and automatically hides high-level epics.

- **_Product Backlog_**  
The holding pool for all unassigned tasks, ideas, and future features.

## Product Backlog
The Product Backlog uses a tree structure to manage requirements, with clear parent-child relationships between levels. From highest to lowest, it is divided into the following three tiers:
 
- **_Epic_**  
Represents high-level business requirements or large feature modules. It typically contains multiple related user stories. Due to its large size, it is expected to be completed gradually over one or more Sprints.

- **_User story_**  
Specific requirements from the user's perspective, helping the team more accurately understand customer goals. To facilitate development, a user story is usually split into multiple specific technical tasks for assignment and execution.

- **_Task_**  
The lowest level of execution. In an ideal state, team members can independently claim the tasks they want to work on. Dependencies and sequential orders between tasks should be minimized to maximize parallel development efficiency.

## Board Workflow & Task Lifecycle
In the Current Sprint board, tasks will strictly flow from left to right through the following status columns:

1. **_User story_**  
The core features to be implemented in the current Sprint, serving as the parent nodes for specific development tasks.

2. **_To do_**  
Tasks that need to be completed but are currently waiting to be claimed.

3. **_In progress_**  
The task has been claimed, and developers are currently writing code or working on it.
(After linking a PR with an Issue, the corresponding Issue card will automatically move from "In progress" to "In review")

4. **In review_**  
A Pull Request has been submitted and is waiting for review by other team members.

5. **_Done_**  
Tasks that have had their PRs successfully merged and meet the team's "Definition of Done" (DoD).

# 💡 Board Usage Guide
**_For Developers_**  
Focus closely on the Current Sprint view. Proactively claim tasks from the Todo column by setting yourself as the Assignee, and drag the card to In Progress when you begin coding.

**_For Sprint Planning Meetings_**   
The Scrum Master will pull high-priority stories from the Product Backlog, assign them the current cycle's Iteration, and ensure they are linked to the correct Epic so the system can automatically aggregate the progress bar.

**_Daily Updates_**  
Please ensure you update the status and progress of your assigned task cards before the start of the Daily Stand-up every day.






