staged area

https://github.com/git/git/blob/master/Documentation/gitformat-index.txt

뭔소린지 모르겠다...

```
// header
DIRC (4bytes) version 2 (4bytes), entries: 4 (32-bit integer), 
```

![[Screenshot 2023-10-09 at 4.16.46 PM.png]]

![[Screenshot 2023-10-09 at 5.53.33 PM.png]]
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

### Cache tree

### Resolve undo

--- 
이 외에도 많음...


## Checksum
sha-1 or sha-256 checksum