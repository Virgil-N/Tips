---
Author: Virgil-N
Date: 2021-6-28 10:14
---

#

## TIPS

- 生成KeyStore：`"C:\Program Files\Java\jdk-14.0.1\bin\keytool.exe" -genkey -alias demo -keypass 123123 -keyalg RSA -keysize 2048 -validity 366 -keystore ./deom.keystore -storepass 321321`
- 查看KeyStore：` "C:\Program Files\Java\jdk-14.0.1\bin\keytool.exe" -list -v -keystore debug.keystore`
- 配置平台支持： `flutter config --enable-windows-desktop`, `flutter config --enable-macos-desktop`, `flutter config --enable-linux-desktop`
- 为已有的应用添加桌面支持： `flutter create --platforms=windows,macos,linux`
- 命令行启动您的应用,， `-d`设置目标平台， `flutter run -d windows`
- 要生成 release 版本： `flutter build windows`