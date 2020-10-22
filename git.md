---
Author: Virgil-N
Date: 2020-06-09T19:52:58+08:00
---
#

## KEEP IN MIND

- 创建本地分支后，提交本地分支并创建远程分支: git push origin branchName:remoteBranchName

- 将本地分支与远程分支关联: git branch --set-upstream-to=origin/somewhere somewhere

- 列出所有tag: git tag

- 新建tag: git tag -a v1.0 -m "first version"

- 删除tag: git tag -d v1.0

- 提交所有tag: git push origin --tags

- 提交某个tag: git push origin v1.0

- 修改git仓库地址: git remote set-url origin https://gitee.com/jouypub/json.git
