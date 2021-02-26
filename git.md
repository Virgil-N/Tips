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

### 设置git代理：

- git config --global https.proxy http://127.0.0.1:1080
- git config --global https.proxy https://127.0.0.1:1080
- git config --global --unset http.proxy
- git config --global --unset https.proxy
- npm config delete proxy
- git config --global http.proxy 'socks5://127.0.0.1:1080'
- git config --global https.proxy 'socks5://127.0.0.1:1080'

#### 或者 path/to/install/Git/etc/bash.bashrc文件修改:
- export http_proxy="http://127.0.0.1:1080/pac?auth=xxxx"
- export https_proxy="http://127.0.0.1:1080/pac?auth=xxxx"

