---
Author: Virgil-N
Date: 2020-06-09T19:52:58+08:00
---
#

## KEEP IN MIND

- 创建本地分支后，提交本地分支并创建远程分支: git push origin branchName:remoteBranchName
- 将本地分支与远程分支关联: git branch --set-upstream-to=origin/remote_branch local_branch
- 列出所有tag: git tag
- 新建tag: git tag -a v1.0 -m "first version"
- 删除tag: git tag -d v1.0
- 提交所有tag: git push origin --tags
- 提交某个tag: git push origin v1.0
- 将本地仓库和远程仓库关联: git remote add origin git@github.com:YotrolZ/helloTest.git
- 修改git仓库地址: git remote set-url origin https://gitee.com/jouypub/json.git
- 强制合并: git pull --allow-unrelated-histories (解决fatal: refusing to merge unrelated histories)

### 基本操作

#### 本地分支

- 创建本地分支，然后切换到dev分支
git checkout -b dev
git checkout命令加上-b参数表示创建并切换，相当于以下两条命令：
git branch dev
git checkout dev

- 然后，用git branch命令查看当前分支：
git branch
* dev
  master

- 添加文件 "Readme.txt", 提交到本地dev分支
git add Readme.txt 
git commit -m "branch dev test"

- dev分支的工作完成，切换回master分支：
$ git checkout master

- 把dev分支的工作内容合并到master分支上：
git merge dev
git merge命令用于合并指定分支到当前分支。合并后，再查看Readme.txt的内容，和dev分支的最新提交是完全一致的。

- Fast-forward信息代表：“快进模式”，直接把master指向dev的当前提交，合并速度快。并非每次代码合并都能实现Fast-forward。

- 合并完成后，删除dev分支：
git branch -d dev

- 删除后，查看branch，就只有master分支了：
git branch
* master

- 创建、合并和删除分支非常快，鼓励你使用分支来完成某个短期任务，合并后再删掉，比起直接在master上工作过程更安全。

####  远程分支

- 删除远程分支
git push origin --delete Chapater6 

- 查看远程分支 
git branch -r

- 拉取远程分支并创建本地分支
git checkout -b 本地分支名x origin/远程分支名x
这样远程仓库中也就创建了一个test分支

git checkout -b test

git push origin test

- 查看所有分支
git branch -a

- 注：remote/origin/[name]表示的是远程分支

#### 清除本地更改

- 清除所有更改
git checkout . && git clean -xdf

- 清除某一个文件的更改
git checkout -- file


### 设置git代理

- git config --global https.proxy http://127.0.0.1:1080
- git config --global https.proxy https://127.0.0.1:1080
- git config --global --unset http.proxy
- git config --global --unset https.proxy
- npm config delete proxy
- git config --global http.proxy 'socks5://127.0.0.1:1080'
- git config --global https.proxy 'socks5://127.0.0.1:1080'

#### 或者 path/to/install/Git/etc/bash.bashrc文件修改

- export http_proxy="http://127.0.0.1:1080/pac?auth=xxxx"
- export https_proxy="http://127.0.0.1:1080/pac?auth=xxxx"

### 问题解决方案

- 出现fatal: refusing to merge unrelated histories，在命令后添加`--allow-unrelated-histories`
