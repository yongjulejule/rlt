# rlt 

Let's implement the git command to learn the internals of git..

## TODO 

- git 은 Plumbing Commands 와 Porcelain Commands 로 나뉜다. (`git help -av | man git`)

### git Plumbing Commands (Low-level Commands)

- [ ] `git hash-object`
- [ ] `git cat-file`
- [ ] `git update-index`
- [ ] `git write-tree`
- [ ] `git read-tree`
- [ ] `git ls-files`

...

### git Porcelain Commands (High-level Commands)

- [ ] `git add`
- [ ] `git commit`
- [ ] `git status`
- [ ] `git log`
- [ ] `git diff`
- [ ] `git push`
- [ ] `git init`

...

---

## NOT SUPPORTED

- `:magic` pattern in pathspec
- rlt only supports the .gitignore file for ignore patterns; other methods like .git/info/exclude are not supported.

## References

- https://wyag.thb.lt/
- https://git-scm.com/book/ko/v2/Git의-내부-Plumbing-명령과-Porcelain-명령
- https://dev.to/unseenwizzard/learn-git-concepts-not-commands-4gjc

