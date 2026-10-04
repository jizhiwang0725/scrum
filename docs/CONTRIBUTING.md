# 🌍 Language Policy 
**Project Management** (Issues/Kanban) is conducted in **Chinese** :cn:.  
**Version Control** (Commits/Branches) must be in **English** :gb:.

# Team Workflow Rule 
## :warning: Main Branch Protection
Direct pushes to the `main` branch are strictly prohibited. All code changes must be made on separate feature branches and merged into the main branch via a Pull Request (PR) after a Code Review.
  
## Branch Naming Convention
`<type>/<short-description>`
  - `feat/user-login`
  - `fix/nav-bar-bug`
  - `docs/update-readme`

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



<style>
/* =========================================
   Base Elements
   ========================================= */
body {
  /* Background */
  color: #e0e0e0;
  padding: 20px;

  /* Font */
  font-family: Tahoma, sans-serif;
  font-size: 1.2rem;
  
}

p {
  margin-bottom: 0rem; 
}

strong {
  /* Text */
  color: #E9C46A;
}

ul {
  padding-left: 20px;
  padding-top: 0px;
}
li > ul {
  padding-left: 15px;
}

blockquote {
  padding-left: 5px;
  margin-right: 5px;
  border-radius: 5px;
}

a {
  color: #e0e0e0 !important;
}

/* =========================================
   Headings
   ========================================= */

h1 {
  margin-top: 1.5em;
}

h2 {
  /* Background */
  margin-top: 0.5em;

  /* Border */
  border-bottom: 3px solid #E9C46A !important;
}

h3 {
  /* Text */
  font-size: 1.5rem;

  /* Background */
  padding: 5px 5px 5px 10px; /* Top, Right, Bottom, Left */

  /* Border */
  border-top: 2px dashed #555555;
  border-bottom: 2px dashed #555555;
}

/* =========================================
   Tables
   ========================================= */
table {
  /* Background */
  display: block; 
  width: max-content; 
  max-width: 100%; 
  padding: 5px;
  
  /* Scroll */
  overflow-x: auto; 
  white-space: nowrap; 
}

</style>






