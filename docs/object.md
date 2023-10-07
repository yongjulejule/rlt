# Blob

- 각 파일이라고 생각하면 된다 like inode.

```
> git cat-file blob <blob-object-hash>
파일 내용1
파일 내용2
파일 내용3
```

# Tree

- 파일들을 모아놓은 것.

```
> git cat-file tree <tree-object-hash>
100644 blob 8cd7bbd3ee28486871392097a861838aa65e272e    .gitenv
040000 tree e1d690b820799b1a0e16535c5eb2ba496de36e10    .github
100644 blob d4a66144edd22a750ba844a836407faeb5584ca4    .gitignore
```

`<mode><sp><type><sp><object-hash><tab><filename>`
# Commit

- tree / parent / author ... 가 적힌 object
```
tree fe41ec9da76c0d0b596ca69faf16e1c54aa3b56a
parent 7018b92003eac4b539e2bcd65ee47c6e6c92c560
author yongjulejule <lyjshow200@gmail.com> 1696676985 +0900
committer yongjulejule <lyjshow200@gmail.com> 1696676985 +0900
	
Feat: Beautiful Commit message
```

# Tags
