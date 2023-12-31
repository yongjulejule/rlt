# rlt 

Let's implement the git command to learn the internals of git..

## TODO 

> git 은 Plumbing Commands 와 Porcelain Commands 로 나뉜다. (`git help -av` or `man git` 참고)

### git options

- [x] `-C` 
- [ ] `--git-dir`
- [ ] `--work-tree`

### git Plumbing Commands (Low-level Commands)

- [ ] `git hash-object`
  - [x] with `-w` option
  - [x] with `<type> <hash>` argument
- [ ] `git cat-file`
  - [x] without options
- [ ] `git ls-files`
  - [x] without options
- [ ] `git check-ignore`
  - [x] without options
- [ ] `git ls-tree`
  - [x] with `recursive` option
  - [x] with path argument, without recursive option
- [ ] `git update-index`
- [ ] `git write-tree`
- [ ] `git read-tree`

...

### git Porcelain Commands (High-level Commands)

- [ ] `git init`
  - [x] without options
- [ ] `git add`
- [ ] `git commit`
- [ ] `git status`
  - [x] without options
- [ ] `git log`
  - [x] without support for packed-refs
- [ ] `git diff`
- [ ] `git push`
- [ ] `git pull`
- [ ] `git clone`

...

---

## NOT SUPPORTED

- `:magic` pattern in pathspec
- rlt only supports the .gitignore file for ignore patterns; other methods like .git/info/exclude are not supported.

## References

- https://wyag.thb.lt/
- https://git-scm.com/book/ko/v2/Git의-내부-Plumbing-명령과-Porcelain-명령
- https://dev.to/unseenwizzard/learn-git-concepts-not-commands-4gjc

