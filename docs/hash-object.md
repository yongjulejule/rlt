# git hash-object `file`

<details>
<summary>  Thanks Copilot Chat! </summary>

git hash-object는 파일의 객체 ID를 계산하는 명령입니다. 이 명령을 사용하려면 계산하려는 파일의 경로를 지정해야 합니다. 따라서 당신의 경우, `file`을(를) 계산하려는 실제 파일 경로로 대체해야 합니다. 예를 들어, 파일이 example.txt이고 현재 디렉토리에 위치한 경우, git hash-object example.txt 명령을 실행해야 합니다.

git hash-object 명령에는 동작을 수정하는 데 사용할 수 있는 몇 가지 옵션이 있습니다. 다음은 가장 일반적으로 사용되는 몇 가지 옵션이다:

-t `타입`: 해시할 오브젝트의 타입을 지정한다. 기본 유형은 파일에 사용되는 "blob"입니다. 다른 유형으로는 "tree"(디렉터리의 경우), "commit"(커밋의 경우), "tag"(태그의 경우) 등이 있습니다.

-w: 개체를 Git 개체 데이터베이스에 씁니다. 기본적으로 git hash-object는 오브젝트 ID를 데이터베이스에 쓰지 않고 계산만 한다.

-stdin: 파일 대신 표준 입력에서 오브젝트 데이터를 읽습니다. 즉석에서 생성된 데이터를 해싱하는 데 유용할 수 있습니다.

-p `객체`: 파일 대신 지정된 객체의 내용을 해시합니다. 트리 또는 커밋 오브젝트를 해싱할 때 유용합니다.

-s `string`: 파일 대신 지정된 문자열을 해시합니다. 파일에 저장되지 않은 데이터를 해싱할 때 유용할 수 있습니다.

이러한 옵션은 원하는 동작을 얻기 위해 필요에 따라 조합할 수 있습니다.

</details>

# git object 만들기

1. 적절한 Content 작성
2. Content length 를 이용해 "`object-type` <Content-length`\\0<Content`" 형태로 문자열 생성
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
