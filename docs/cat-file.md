```
usage: git cat-file <type> <object>
   or: git cat-file (-e | -p) <object>
   or: git cat-file (-t | -s) [--allow-unknown-type] <object>
   or: git cat-file (--batch | --batch-check | --batch-command) [--batch-all-objects] [--buffer] [--follow-symlinks] [--unordered] [--textconv | --filters] [-Z]
   or: git cat-file (--textconv | --filters) [<rev>:<path|tree-ish> | --path=<path|tree-ish> <rev>]
```

이거 시놉시스가 왜케 빡쌤? 

# git cat-file 


## \<type>

- blob
- tree
- commit
- tag
- 

## \<object>

- Hash 값. gitrevisions 참고해야함 (나중에 ㅎ)

## Options

### -t

Object 의 타입을 알려줌

### -s 

Object 의 size 를 알려줌
### -e

해당 object 가 있으면 0, 없으면 1 로 exit

### -p

object 의 타입에 맞게 이쁘게 출력해줌

(--batch | --batch-check | --batch-command) [--batch-all-objects]
                    [--buffer] [--follow-symlinks] [--unordered]
                    [--textconv | --filters] [-Z]

%% TODO: rev-parse , revision 후 보기 

<details>
<summary> Batch 관련해선 rev-parse 보고 해야할듯... </summary>

### --batch, --batch=`format`
   •   When used with --textconv or --filters, the input lines must specify the path, separated by whitespace. See the section BATCH OUTPUT below
               for details.

   •   When used with --use-mailmap, for commit and tag objects, the contents part of the output shows the identities replaced using the mailmap
               mechanism, while the information part of the output shows the size of the object as if it actually recorded the replacement identities.

- stdin 으로 받은 각 Object 의 정보와 내용을 출력. 
- --textconv, --filters, --use-mailmap 옵션과만 조합할 수 있음

Example
- echo -e "HEAD\nHEAD~1" | git cat-file --batch 

### --batch-check

### --batch-command

### --batch-all-objects

### --buffer [--unordered]

### --textconv

### ---filters

</details>
