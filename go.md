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

### 编译

- 运行go build命令，只需要将生成的可执行文件及资源目录以当前相对的路径拷贝放到其他任何目录下即可运行

### fyne

- 静态资源编译进文件
  - eg: ```fyne bundle -package assets -prefix Resource  ./resource/img/icon/appIcon.jpg > ./assets/bundle.go```

- 打包apk文件
  - eg: ```fyne package -os android -name fyne_app -appID com.samete.fyne_app -icon ./resource/img/icon/appIcon.jpg```

### Android

- ADB保存日志
  - eg: ```adb logcat -v time > log.txt```
