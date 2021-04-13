---
Author: Virgil-N
Date: 2020-01-04T19:52:58+08:00
---
#

## KEEP IN MIND

- 如果在下载一些墙外的包可设置代理:

```
export GOPROXY="https://athens.azurefd.net"
export GO111MODULE=on
```

- 使用go mod默认检查域名形式的路径，可采用replace解决，如下：

```go
require github.com/Virgil-N/testApp v1.0.0

replace github.com/Virgil-N/testApp => ./testApp
```

- 如果有嵌套的情况，顶层mod文件还得加上replace语句

---

### 项目目录结构

/cmd
main函数文件（比如 /cmd/myapp.go）目录，这个目录下面，每个文件在编译之后都会生成一个可执行的文件。

不要把很多的代码放到这个目录下面，这里面的代码尽可能简单。

/internal
应用程序的封装的代码，某个应用私有的代码放到 /internal/myapp/ 目录下，多个应用通用的公共的代码，放到 /internal/common 之类的目录。

/pkg
一些通用的可以被其他项目所使用的代码，放到这个目录下面

/vendor
项目依赖的其他第三方库，使用 glide 工具来管理依赖

/api
协议文件，Swagger/thrift/protobuf 等

/web
web服务所需要的静态文件

/configs
配置文件

/init
服务启停脚本

/scripts
其他一些脚本，编译、安装、测试、分析等等

/build
持续集成目录

云 (AMI), 容器 (Docker), 操作系统 (deb, rpm, pkg)等的包配置和脚本放到 /build/package/ 目录

/deployments
部署相关的配置文件和模板

/test
其他测试目录，功能测试，性能测试等

/docs
设计文档

/tools
常用的工具和脚本，可以引用 /internal 或者 /pkg 里面的库

/examples
应用程序或者公共库使用的一些例子

/assets
其他一些依赖的静态资源

---

### 编译

- 运行go build命令，只需要将生成的可执行文件及资源目录以当前相对的路径拷贝放到其他任何目录下即可运行

- 在Linux和Mac系统上编译wasm文件，命令为：`GOARCH=wasm GOOS=js go build -o web/app.wasm`，Windows系统上为：`$env:GOARCH="wasm";$env:GOOS="js"; go build -o web/app.wasm`

### fyne

- 静态资源编译进文件
  - eg: ```fyne bundle -package assets -prefix Resource  ./resource/img/icon/appIcon.jpg > ./assets/bundle.go```

- 打包apk文件
  - eg: ```fyne package -os android -name fyne_app -appID com.samete.fyne_app -icon ./resource/img/icon/appIcon.jpg```

### Android

- ADB保存日志
  - eg: ```adb logcat -v time > log.txt```
