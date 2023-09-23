```
SYNOPSIS
	git init [-q | --quiet] [--bare] [--template=<template-directory>]
	[--separate-git-dir <git-dir>] [--object-format=<format>]
	[-b <branch-name> | --initial-branch=<branch-name>]
	[--shared[=<permissions>]] [<directory>]

```

뭐가 너무 많다. 일단 옵션 없이 git init 부터 생각해보자

![[Screenshot 2023-09-21 at 8.26.10 PM.png]]


솔찍히 hook sample 파일들 제공하는거 너무 친절한것 같다. 그리고 제공 해봤자 대부분 관심없고 husky 보고 hook 관리하라 한다

# Config

config 를 저장하는 파일. 첫 생성시는 다음과 같다.

```
$> cat config
[core]
	repositoryformatversion = 0
	filemode = true
	bare = false
	logallrefupdates = true
	ignorecase = true
	precomposeunicode = true
```


자세한 설명은 git-config(1) 에 있다. 구현할때 생각해보자 ㅎ

# HEAD
```
$> cat HEAD
ref: refs/heads/main
```

HEAD Branch 를 저장하는 파일 . HEAD  는 뭘까?
> HEAD : indicates the head of the current branch. (from git(1))

그렇다. 기본 브랜치 설정에 따라 내용이 달라질 것 같이 생겼다.

ref: refs/heads/<SOME_BRANCH>

근데 꼭 branch 여야 한가? 

걍 commit 에 checkout 하고 HEAD 파일을 보면 
```
$> git checkout 4eb360fe1ad41f2cf1a602ba76ac0b55032960a8
$> cat .git/HEAD
4eb360fe1ad41f2cf1a602ba76ac0b55032960a8
```
와 같다. branch 면 reference 여서 ref: refs/heads/blah-blah 가 되고 hash 면 걍 입력되는듯? 

근데 refs/heads/main 이 너무 파일같이 생기지 않았나?

```
$> cat refs/heads/main
4eb360fe1ad41f2cf1a602ba76ac0b55032960a8
```

헉 이걸맞추네;  근디 main 파일이 존재하는건 main branch 에 commit 이 있어서 그런듯
`.git` 날리고 다시 `git init` 하면 여전히 HEAD 는 `ref: refs/heads/main` 이지만, `main` 이라는 파일은 존재하지 않는다. 걍 write 하는 방식인듯? 그게맞지

# Description 

이거 어따씀? 몰라... github 의 description 랑도 다름
 >The `description` file is used only by the GitWeb program, so don’t worry about it
 >https://git-scm.com/book/en/v2/Git-Internals-Plumbing-and-Porcelain
 
 그렇다고 합니다... 뭐 github 만 있는게 아니니까.

# objects

git-objects 가 저장되는데, 일단은 디렉토리만 만들어주면 된다. 
아마 blob / tree / commit 때 중요하게 볼듯

# refs

git-branchs  or git-tags 를 관리하는 곳인데, 아직 잘 모른다. 디렉토리만 만들면 됨.

# info
`exclude` 파일이 있는데, .gitignore 와 역할이 같다고 한다. 단지 git tracking 이 안될뿐. 좋네? 이런게 있다니 굿이네요


---
위 파일들을 그냥 생성하기만 하면 된다.


# options

## [-q | --quiet]
stdout 을 날린다.
## --bare
날것으로 repo 를 만든다. GIT_DIR 환경변수가 없으면 걍 현재 디렉토리에 .git 에 있어야 할 아이들을 갖다박음
## --template <template_directory>
.git 에 template_directory 에 있는 파일들도 다갖다 박는다. 왜씀?;

## --separate-git-dir <git_dir>

다른 디렉토리를 .git 으로 쓴다. 현재 디렉토리에 .git 이 생기는데 
```
$> cat .git
gitdir: /Users/jun/goinfre/git-testing
```
일케 저장된다.
## --object-format \<format\>
sha1 | sha256 를 받아서 해시함수를 정함
```
$> cat config
[core]
	repositoryformatversion = 1
	filemode = true
	bare = false
	logallrefupdates = true
	ignorecase = true
	precomposeunicode = true
[extensions]
	objectformat = sha256
```
일케 관리됨
## \<directory\>

해당 디렉토리에서 git init 을 함