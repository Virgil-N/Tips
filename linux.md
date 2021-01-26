---
Author: Virgil-N
Date: 2020-10-04T19:52:58+08:00
---

#

## LINUX

- window ssh连接linux命令: ssh account@ip
- 查看某个应用的进程及结束进程: 命令窗口输入 `ps -ef | grep '应用名'`
- 使用 `kill -9  进程号` 强制杀死该进程
- 查看端口是否开放: `netstat -ntpl grep 端口号` (TCP类型), `netstat -nupl grep 端口号` (UDP类型)
- -r 就是向下递归，不管有多少级目录，一并删除
- -f 就是直接强行删除，不作任何提示的意思
- 删除文件夹实例：
rm -rf /var/log/httpd/access
将会删除/var/log/httpd/access目录以及其下所有文件、文件夹
- 删除文件使用实例：
rm -f /var/log/httpd/access.log
将会强制删除/var/log/httpd/access.log这个文件
- -m 分配权限
- -p 指定路径
例: mkdir -m 711 test2
- scp 复制命令: scp -r(加-r代表复制目录) 本地主机目录或文件路径 biny@192.168.1.220:~(~代表用户根目录，也可以使用其他目录)
- cp 命令: cp -r 源文件目录 目标路径

## linux 中 nginx

- 日志目录 /var/log/nginx
- 文件目录 /usr/share/nginx/html
- 配置目录 /etc/nginx