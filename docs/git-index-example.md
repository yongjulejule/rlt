
# Tree extension

- `mkdir a && touch a/b && git add . && git commit -m "test" && cat .git/index | hexdump -C` 의 결과

```
00 00 00 00 54 52 45 45  00 00 00 33 00 31 20 31
|....TREE...3.1 1|
0a 87 8e 27 c6 26 26 6a  c0 40 87 a2 03 e4 bd d3
|...'.&&j.@......|
96 dc f7 47 63 61 00 31  20 30 0a 42 77 b6 e6 9d
|...Gca.1 0.Bw...|
25 e5 ef a7 7c 45 53 40  55 7b 38 4a 4c 01 8a fa
|%...|ES@U{8JL...|
d3 c9 0b f0 de 68 58 50  df 00 60 2f 64 7b 30 64
|.....hXP..`/d{0d|
51 6d 2b
|Qm+|
```

- 00 00 00 00 54 52 45 45
	- `paddings> TREE` (signature)
- 00 00 00 33
	- size of this extension (51)
- 00 31 20 31 0a
	- `<dir>\0<entry_count> <sp> <the number of subtrees> <newline>`
	- 현재 dir (null-terminating), entry_count : 1, space, the number of subtree: 1, newline (0a)
- 878e27c626266ac04087a203e4bdd396dcf74763
	- 현재 tree object 의 sha1 hash 값 (현재 commit 의 tree 에 적힌 hash 와 같음)
- 61 00 31  20 30 0a
	- `<dir>\0<entry_count> <sp> <the number of subtrees> <newline>`
	- a 라는 dir, entry_count: 1, space, 0 number of subtree, newline
- 4277b6e69d25e5efa77c455340557b384a4c018a
	- a tree 의 hash 값
	- `git rev-parse HEAD:a` 와 같음
- fad3c90bf0de685850df00602f647b3064516d2b
	- index file checksum. 이전 모든 부분을 갖고 sha1 hash 갈기면 나오는 값. `head -c 143 .git/index | openssl sha1` 와 같음


