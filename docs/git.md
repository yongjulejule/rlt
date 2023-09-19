# git 은 어쩌구

우선 Git은 기본적으로 Content-addressable 파일 시스템이고 그 위에 VCS 사용자 인터페이스가 있는 구조다. (이게머람)

- VCS: version control system
- Content Addressable Storage(CAS): 컨텐츠에 대한 주소를 만들어서 관리한다...는 머 그런 뜻인듯? 역시 세상은 포인터다 [Wikipedia](https://en.wikipedia.org/wiki/Content-addressable_storage)

## git internal 

- git objects 
  - blob
    - 대충 파일 내용을 sha-1 로 빼서 k-v 로 저장한거 -> unix 의 inode 나 normal file 같은거
  - tree 
    - Blob 과 유사한데 파일 이름들을 저장한거. -> directory 같은거

- Git 은 왜 아직도 sha-1 을 쓸까?
	- https://lwn.net/Articles/898522/
	- sha-256 이 지원되긴 한다고 함.
	- 근데... 참 애석하게도 하위호환성, git forge 제공자와 상호 운용성이 떨어진다고 함. 
	- https://lwn.net/Articles/823352/ - sha 256 update 게시글. 여러 의견이 있는듯
	- https://git-scm.com/docs/hash-function-transition/ sha-1 -> other hash function 으로 migration 에 관한글 
	- git init --object-format sha256 을 통하여 sha 256 으로 관리되는 repository 를 만들 수 있음.



## git commands

###  -v | --version
print version
### -h | --help
show help message

###  -C \<path\>

path 에서 git command 를 실행. -C "" 일케 오면 걍 현재 working directory. 
아래 두 커멘드는 같은 역할을 함
```
git --git-dir=a.git --work-tree=b -C c status
git --git-dir=c/a.git --work-tree=c/b status
```

### --work-tree=\<path\>

이건 먼소린지 몰겟음. git-config & git-worktree 를 참고하라는데 담에 볼게...

###  --git-dir=\<path\>

기본적으로 .git 으로 관리되는 그 디렉토리를 변경. --git-dir option 대신 GIT_DIR 이란 환경변수로도 control 할 수 있음.


## git init & git clone

- .git 이라는 directory 가 생김
- .git 의 HEAD 유무로 git repository 인지 아닌지 판단 하는듯 ???

## 나머지

- git subcommand 를 입력하면 any parent directory 를 탐색하며 git repository 를 찾는다 (except --help | -h)
