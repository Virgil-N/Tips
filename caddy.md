---
Author: Virgil-N
Date: 2020-08-04T19:52:58+08:00
---
#

## KEEP IN MIND

## 以下是项目的配置，接口请求使用代理

```
  localhost:2020 {
    encode gzip

    header {
      Strict-Transport-Security "max-age=31536000;"
      X-XSS-Protection "1; mode=block"
      X-Content-Type-Options "nosniff"
      X-Frame-Options "DENY"
    }

    header /api/* {
      Access-Control-Allow-Origin  *
      Access-Control-Allow-Methods "GET, POST, DELETE, PUT, OPTIONS, PATCH"
    }

    file_server {
      root C:/mySoftware/caddy/web/
      index /index.html
    }

    @notIndex {
      expression {path}.matches("(login)|(home)")
    }

    route {
      rewrite @notIndex /index.html
    }

    route /api/* {
      uri strip_prefix api
      reverse_proxy http://218.205.107.96:8099
    }

    log {
      output file         AOSH.log
      format single_field common_log
    }
  }
```
