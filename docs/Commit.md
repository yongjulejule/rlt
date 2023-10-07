commit object 는 일케 생겼다.

```
tree dc8184b9c5ed36f64f24cbd91625f6f697bdd7c3
parent 44f6b03323cf8e5a99fe801963bb578bc79fbed9
author yongjulejule <lyjshow200@gmail.com> 1695222915 +0900
committer yongjulejule <lyjshow200@gmail.com> 1695222915 +0900

:memo: docs: Add docs. unix socket can not be added on git 😲
```

```
tree 6befc04c056ae3a514b87d41cf13833cbd8320a7
parent 84d3eac87e1a848679bf08e0fef3d53f5b35f059
parent b78e69c4d8904de7698b9d880298ee8db6e558e3
author Yongjun Lee <lyjshow200@gmail.com> 1680506819 +0900
committer GitHub <noreply@github.com> 1680506819 +0900
gpgsig -----BEGIN PGP SIGNATURE-----
 
 BLAHBLAH
 BLAHBLAH
 BLAHBLAH
 ...
 -----END PGP SIGNATURE-----
 

이렇게 생긴 친구도 있다.
```

- tree 는 걍 디렉토리 같은거라 생각하면 되고, commit 에 해당하는 tree-object 의 hash 값이 저장된다.
- parent 는 해당 커밋의 parent 이며, merge commit 같은 경우에 2개 이상 생길 수 있다.  또한 initial commit 같은 경우엔 parent 가 없다. 

git log?
.git/HEAD -> .git/refs/heads/main -> object-hash (top object)
and then, keep track of parent