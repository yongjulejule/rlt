# status

- 다음 세 영역의 파일들을 비교
    - staged file
        - ls-files 와 같음
    - HEAD
        - ls-tree -r 과 같음
    - local file-system
        - ignore 체크 후 걍 local file 읽어야함


- HEAD tree 에 저장된 파일의 hash 와index 에 저장된 파일의 hash 가 다르다?
>Changes to be committed:
>   (use "git restore --staged <file>..." to unstage)
>   modified:   src/adapters/command_executor.rs

- index 에 저장된 파일의 hash 와 로컬 파일의 hash 가 다르다?
>Changes not staged for commit:
>   (use "git add <file>..." to update what will be committed)
>   (use "git restore <file>..." to discard changes in working directory)
>   modified:   src/main.rs
        
- index 에 저장된 파일 목록에 해당 파일이 존재하지 않음
>Untracked files:
>   (use "git add <file>..." to include in what will be committed)
>   docs/status.md

