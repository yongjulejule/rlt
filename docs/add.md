# git add 를 하면 무슨일이 일어날까?

-  .git/index 파일에 추가된 파일의 정보가 저장된다.
- .git/objects 에 blob object 가 생성된다.

# git object 만들기

1. 적절한 Content 작성
2. Content length 를 이용해 "\<object-type\> <Content-length\>\\0<Content\>" 형태로 문자열 생성
3. 생성한 문자열을 이용해 sha-1 hash
4. hash 한 문자 앞 2글자를 디렉토리,  나머지 38글자를 (sha-256 이면 62 글자) 를 파일명으로 파일 저장
5. Content 를 zlib compress 를 해서 파일 내용에 write
6. git cat-file -p 로 결과 확인!


```
Content = "git is same to rlt";
Header = "blob Content.length\0";
Sha1Hashed = sha1.hash(Header + Content);
compressed = zlib.compress(Hased)
path = '.git/objects/' + Sha1Hashed[0..1] + Sha1Hashed[2..]
createFile(path).write(compressed)
```

