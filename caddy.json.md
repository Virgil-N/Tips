#

### 实例配置

```json
{
  "admin": {
    "disabled": false,
    "listen": "127.0.0.1:2021",
    "enforce_origin": false,
    "origins": ["127.0.0.1:2021"],
    "config": {
      "persist": false
    }
  },
  "apps": {
    "http": {
      "servers": {
        "app1": {
          "listen": ["127.0.0.1:2021"],
          "read_timeout": "5m",
          "read_header_timeout": "3m",
          "idle_timeout": "5m",
          "routes": [
            {
              "handle": [
                {
                  "handler": "file_server",
                  "root": "./web",
                  "hide": [],
                  "index_names": ["index.html"]
                }
              ]
            }
          ],
          "automatic_https": {
            "disable": false,
            "disable_redirects": false,
            "skip": [],
            "skip_certificates": [],
            "ignore_loaded_certificates": false
          },
          "strict_sni_host": false,
          "logs": {
            "default_logger_name": "",
            "logger_names": {
              "": ""
            },
            "skip_hosts": [],
            "skip_unmapped_hosts": false
          },
          "experimental_http3": false,
          "allow_h2c": false
        }
      }
    }
  }
}

```
