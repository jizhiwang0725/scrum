<link rel='stylesheet' href='./assets/style.css'>

<script>
// A tiny delay ensures VS Code has finished rendering the HTML elements
setTimeout(() => {
   const expandableItems = document.querySelectorAll('.toc-sidebar > ul > li > ul > li:has(> ul)');

   expandableItems.forEach(item => {
       item.addEventListener('click', function(event) {
           // Ensure the click happened directly on the list item or arrow
           if (event.target.tagName !== 'A' || event.target.parentElement === this) {
               this.classList.toggle('is-open');
           }
       });
   });
}, 300); // Waits 300 milliseconds before attaching the clicks
</script>


<div class='toc-sidebar'>

<strong>Table of Contents</strong>

- [Team Workflow Rule](#team-workflow-rule)
  - [Main Branch Protection](#main-branch-protection)
  - [Branch Naming Convention](#branch-naming-convention)
    - [Structure](#structure)
- [Pull Request Standards](#pull-request-standards)
  - [Naming Convention](#naming-convention)
  - [Templates](#templates)
- [Committing Message](#committing-message)
  - [One-line](#one-line)
    - [Structure](#structure-1)
    - [Type](#type)
    - [Scope](#scope)
    - [Subject](#subject)
  - [Multi-line](#multi-line)
- [Committing Criteria](#committing-criteria)
  - [Atomic Commits](#atomic-commits)
- [Integration \& Conflict Resolution](#integration--conflict-resolution)
  - [Defensive committing](#defensive-committing)
    - [Instructions](#instructions)
- [Workflow Closing](#workflow-closing)
  - [Local Cleanup After PR](#local-cleanup-after-pr)

</div>

<div class= "main-content">


# Team Workflow Rule 
## Main Branch Protection
- **Strict Rule:** Direct pushes to the `main` branch are strictly prohibited. 
- All code changes must be made on separate feature branches and merged into the main branch via a Pull Request (PR) after a Code Review.
  
## Branch Naming Convention
### Structure
- `<type>/<short-description>`
- **Examples:** 
  - `feat/user-login`
  - `fix/nav-bar-bug`
  - `docs/update-readme`

# Pull Request Standards
## Naming Convention 
- Follow the same format used in one-line commits 
  `<type>(<scope>): <subject>`

## Templates 
- Require developers to fill out the **checklist in the PR description**, expanding on the "why this change was made" concept you currently suggest for complex commits.

# Committing Message 
## One-line 
### Structure
- `<type>(<scope>): <subject>`


### Type

| Type | Meaning | 
| --- | --- |
| `feat` | - A new feature that benefits the users |
| `fix` | - A bug fix | 
| `docs` | - Documentation only changes (like README.md) |
| `style` | - Formatting changes that **do not** affect the code's meaning<br> (white-space, formatting) | 
| `refactor` | - A code change that neither fixes a bug nor adds a feature,<br> just **optimizes structure**
| `test` | - Adding missing tests or correcting existing tests | 
| `chore` | - Changes to build process or auxiliary tools for developers only (e.g. configuring `gitignore`)

### Scope
- The scope indicate **which module you modified** (e.g., (ui) or (database); this can be omitted initially).

### Subject
- Always **start with a lowercase**
- **Concise and clear**, summarizing your changes in a single sentence 
- **Always start the subject with a present verb**
- If the code for a particular commit is extremely complex, you can **leave a blank line after the subject** and write a multiline Body to explain in detail "why this changes was made," though this is usually better suited for the **Pull Request description on GitHub**

## Multi-line
- **Dominated by the Primary Purpose**   
  
  Consists of multiple single-line structure

  For those secondary changes, use the **multi-line Body** provided.  

    ```bash
    feat(backend): add user login module

    - feat: support basic email and password validation
    - fix: resolve layout misplacement in navigation bar
    - docs: update README to add environment dependency instructions
    ```

# Committing Criteria 
## Atomic Commits
- Specify exact files: `git add login.py` then write a `feat` commit; subsequently, execute `git add README.md`then write a `docs` commit.  

# Integration & Conflict Resolution
## Defensive committing 
- You've been writing UI code on your `feat-ui` branch for three days and are ready to push it to the cloud. However, you know perfectly well that other team members must have merged other code into the `main` branch during these three days. To avoid throwing a merge conflict onto GitHub for others to stress over, you decide to **integrate it locally first**

### Instructions
- Ensure your current code is already tested and committed   
- **Stay on your** `feat-ui` **branch**, and directly run `git pull origin main`
- At this point, a bunch of conflicts will likely pop up. Don't panic. Open VS Code, find the files marked in red, click the visual **"Accept Both Changes"** (or which ever option you deem correct), and save the files.
- Commit the resolution
  `git commit -m "chore: resolve merge conflicts with main"`

- Push your integrated code to the cloud: `git push`

# Workflow Closing 
## Local Cleanup After PR
- Once your PR has been reviewed and merged on GitHub, return to your local terminal and execute the following commands to keep your repository clean 

    ``` bash
    # 1. Switch to the main branch and sync the latest team progress
    git switch main
    git pull

    # 2. Clean up local tracking branches that have been deleted on the remote
    git fetch --prune

    # 3. Delete your local feature branch
    git branch -d <your-branch-name>
    ```
</div>








