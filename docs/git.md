# git 은 어쩌구

우선 Git은 기본적으로 Content-addressable 파일 시스템이고 그 위에 VCS 사용자 인터페이스가 있는 구조다. (이게머람)

VCS - version control system
Content addressable (CAS) - 컨텐츠에 대한 주소를 만들어서 관리한다...는 머 그런 뜻인듯? [Wikipedia](https://en.wikipedia.org/wiki/Content-addressable_storage)

## git init & git clone

- .git 이라는 directory 가 생김
- .git 의 HEAD 유무로 git repository 인지 아닌지 판단 하는듯 ???

## 나머지

- git subcommand 를 입력하면 any parent directory 를 탐색하며 git repository 를 찾는다 (except --help | -h)
