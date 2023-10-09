staged area

https://github.com/git/git/blob/master/Documentation/gitformat-index.txt


```
// header
DIRC (4bytes) version 2 (4bytes), entries: 4 (32-bit integer), 
```

![mac-result](<assets/Screenshot 2023-10-09 at 4.16.46 PM.png>)

![debian result](<assets/Screenshot 2023-10-09 at 5.53.33 PM.png>)


# Structure
다음과 같은 구조로 이루어진다.

- Index Headers
	- metadata 를 저장하는 Header field
- Index Entries
	- file 들을 나타내는 entires
- Extensions (Optional)
	- optional extensions
- CheckSum
	-  20 byte SHA-1 checksum

## Header
`DIRC <VERSION> <ENTIRES>` 와 같이 구성된다.
- "DIRC" 는 예약된 HEADER 
- VERSION 은 4-byte 로 보통 2, 3, 4 임
- ENTRIES 는  32 bit integer 로 관리하는 파일의 개수
```
 D  I  R  C  | Version 2   | the number of entries
 44 49 52 43 | 00 00 00 02 | 00 00 00 03
```

## Entries

- entries 는 name 을 기준으로 오름차순으로 정렬됨
- 각 entry 는 다음과 같이 생김
- `<ctime.seconds> <ctime.nanoseconds> <mtime.seconds><mtime.nanoseconds> <dev> <inode> <mode> <uid> <gid> <file-size> <object-name> <flags> <extended flag (version 3 or later)> <path name> <1~8 null byte-padding>`
- ctime: created time. 32bit second, 32bit nanosecond
- mtime: modified time.  32bit second, 32bit nanosecond
- dev: 32bit. device ID for inode 
- inode: 32bit. inode of file
- mode:  32bit.
	- 16 bit - zero. unused
	- 4 bit - object-type
		- regular file: 1000
		- symlink: 1010
		- gitlink: 1110
	- 3bit - zero. unused
	- 9bit - unix permission. 
		- Regular file - 0755 / 0644  
		- Symlink & gitlink - 0
- uid: 32bit
- gid: 32bit 
- file-size: 32bit
- flags: 16 bit
	- 1bit - assume-valid flag
	- 1bit - extended flag (0 in version 2)
	- 2bit - stage (during merge)
	- 12bit - name length (if less than 0xFFF. otherwise, 0xFFF)
- extended flag: 16 bit (only applicable if the "extended flag" above is 1)
	- 1 bit - reserved for future
	- 1 bit - skip-worktree flag (used by sparse checkout)
	- 1 bit - intent to add flag (used by "git add -N")
	- 13 bit - zero. unused
- path name: relative to top level dir without leading slash. `.`, `..`, `.git` are reserved. Trailing slash is disallowed
- <some version 4 feature>
- byte padding - to make multiply of eight
## Extensions

각 extension 은 다음과 같이 구성
`<signature> <size of extension> <Extension data>`
- Signature: 4 byte 의 [A-Z] 로 시작하는 문자. 
- size of extension: 32 bit integer. 
- Extension data ...

```
00000280  73 74 2d 32 33 00 00 00  00 00 00 00 54 52 45 45  |st-23.......TREE|
00000290  00 00 00 61 00 2d 31 20  31 0a 74 65 73 74 00 36  |...a.-1 1.test.6|
000002a0  20 32 0a 21 20 47 9e c8  ba 0b 3f d5 a9 0e 94 90
| 2.! G....?.....|
000002b0  e3 33 3a 8d b4 db 13 74  65 73 74 2d 31 00 33 20  |.3:....test-1.3 |
000002c0  30 0a ae 25 e9 a7 83 25  cd 1e b3 32 2a 5a 1e 65  |0..%...%...2*Z.e|
000002d0  ba 87 12 dc 41 d5 74 65  73 74 2d 32 00 32 20 30  |....A.test-2.2 0|
000002e0  0a 60 46 e3 06 a5 fe 88  34 48 4c ad 71 28 df da  |.`F.....4HL.q(..|
000002f0  9a 2b 59 b2 03 52 45 55  43 00 00 00 53 61 00 31  |.+Y..REUC...Sa.1|
00000300  30 30 36 34 34 00 31 30  30 36 34 34 00 31 30 30  |00644.100644.100|
00000310  36 34 34 00 e6 9d e2 9b  b2 d1 d6 43 4b 8b 29 ae  |644........CK.).|
00000320  77 5a d8 c2 e4 8c 53 91  e2 b2 5e 69 a9 0a a7 2f  |wZ....S...^i.../|
00000330  67 e6 fa 79 dc 15 80 b6  fb 3b 2e 5b 84 61 c1 aa  |g..y.....;.[.a..|
00000340  ca c9 d9 5e 1c 6b d9 a1  0b 41 4d 22 5e 19 2b 14  |...^.k...AM"^.+.|
00000350  3a e6 d8 23 6f c8 03 42  bf 80 12 2d 4c d5 75 16  |:..#o..B...-L.u.|
00000360  e0 a6 40 5b                                       |..@[|
```

### Cache tree

[[git-index-example]]

<details><summary> 번역 </summary> 
=== 캐시 트리

  index 는 디렉터리에 대한 항목을 기록하지 않기 때문에 캐시 항목은 기존 커밋에서 변경되지 않은 인덱스 영역에 대해 오브젝트 데이터베이스에 이미 존재하는 트리 오브젝트를 설명할 수 없다. 캐시 트리 확장은 이미 존재하고 캐시 항목의 섹션과 완전히 일치하는 트리를 설명하는 재귀적 트리 구조를 저장합니다. 이렇게 하면 해당 커밋에 "새로운" 트리만 계산하여 새 커밋에 대한 인덱스에서 트리 오브젝트 생성 속도가 빨라집니다. 또한 트리 비교에서 동일성이 입증되면 인덱스의 섹션을 건너뛸 수 있으므로 `HEAD^{tree}`와 같은 다른 트리와 인덱스를 비교할 때도 도움이 됩니다.

  재귀 트리 구조는 여러 캐시 항목, 하위 노드 목록 및 객체 ID(OID)를 저장하는 노드를 사용합니다. OID는 해당 노드가 존재하는 것으로 알려진 경우 해당 노드의 기존 트리를 참조합니다. 하위 노드는 자체적으로 캐시 트리 노드가 있는 하위 디렉터리에 해당합니다. 캐시 항목 수는 해당 트리의 디렉터리 내 경로를 설명하는 인덱스의 캐시 항목 수에 해당합니다.

  확장 프로그램은 캐시 트리 확장에서 전체 디렉토리 구조를 추적하지만 일반적으로 전체 캐시 항목 목록보다 작습니다.

  인덱스에서 경로가 업데이트되면 Git은 해당 경로의 상위 디렉터리에 해당하는 재귀 캐시 트리의 모든 노드를 무효화한다. 캐시 항목 수에 "-1"을 사용하여 이러한 트리 노드를 "유효하지 않은" 상태로 저장합니다. 유효하지 않은 노드는 여전히 인덱스 항목의 범위를 저장하므로 전체 캐시 트리를 재구성할 때 Git이 작업에 집중할 수 있습니다.

  이 확장의 시그니처는 { 'T', 'R', 'E', 'E' }입니다.

  일련의 항목이 전체 extension 을 채우며, 각 항목은 다음과 같이 로 구성됩니다:

  - NUL로 끝나는 경로 구성 요소(상위 디렉터리에 상대적);
  - 이 항목이 나타내는 트리에 포함되는 인덱스 내 항목의 ASCII 십진수 트리에 포함된 인덱스의 항목 수(entry_count);
  - 공백(ASCII 32);
  - 이 트리가 가진 하위 트리의 수를 나타내는 ASCII 십진수. 트리가 가진 하위 트리의 수를 나타내는
  - 개행(ASCII 10), 그리고
  - 이 인덱스 스팬을 트리로 작성할 때 생성되는 객체의 객체 이름입니다.

  항목은 무효화된 상태일 수 있으며 entry_count 필드에 음수로 표시됩니다. 이 경우 개체 이름이 없으며 다음 항목은 개행 바로 뒤에 시작됩니다. 유효하지 않은 항목을 작성할 때는 항상 -1을 entry_count로 사용해야 합니다.

  항목은 하향식, 깊이 우선 순서로 작성됩니다.  첫 번째 항목은 리포지토리의 루트 레벨을 나타내고, 그 다음에는 루트 레벨의 첫 번째 하위 트리(루트 레벨에 상대적인 이름을 사용함), 그 다음에는 A의 첫 번째 하위 트리(A에 상대적인 이름을 사용함), 그 다음에는 A의 첫 번째 하위 트리(이를 A라고 함)가 차례로 나타납니다. 지정된 수의 하위 트리는 재귀 스택의 현재 레벨이 완료된 시점을 나타냅니다.

</details>

각 Extension 은 다음과 같이 구성됨

- NUL-terminated path component (relative to its parent directory);
- ASCII decimal number of entries in the index that is covered by the tree this entry represents (entry_count);
- A space (ASCII 32);
- ASCII decimal number that represents the number of subtrees this tree has;
- A newline (ASCII 10); and
- Object name for the object that would result from writing this span of index as a tree.
### Resolve undo

--- 
이 외에도 많음...


## Checksum
sha-1 or sha-256 checksum
