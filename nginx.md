---
Author: Virgil-N
Date: 2020-08-04T19:52:58+08:00
---
#

## KEEP IN MIND

- 调用 nginx -s stop 快速停止nginx

- 调用 nginx -s quit 完整有序的停止nginx

- nginx -s reload 平滑的重启，配置重载

- 测试指定配置文件是否正确： nginx -t (指定配置文件路径)

## 以下是项目的配置，接口请求使用代理

```
#user  nobody;
worker_processes  1;

error_log  logs/error.log;
#error_log  logs/error.log  notice;
#error_log  logs/error.log  info;

# pid        logs/nginx.pid;


events {
    worker_connections  1024;
}


http {
    include       mime.types;
    default_type  application/octet-stream;

    #log_format  main  '$remote_addr - $remote_user [$time_local] "$request" '
    #                  '$status $body_bytes_sent "$http_referer" '
    #                  '"$http_user_agent" "$http_x_forwarded_for"';

    #access_log  logs/access.log  main;

    client_max_body_size 300m;
    sendfile        on;
    #tcp_nopush     on;

    #keepalive_timeout  0;
    keepalive_timeout  300;

    send_timeout 300;

    proxy_connect_timeout 300s;
    proxy_send_timeout 300s;
    proxy_read_timeout 300s;

    fastcgi_connect_timeout 300;
    fastcgi_send_timeout 300;
    fastcgi_read_timeout 300;

    fastcgi_buffers 2 256k;
    fastcgi_buffer_size 128k;
    fastcgi_busy_buffers_size 256k;
    fastcgi_temp_file_write_size 256k;

    gzip  on;

    # upstream aosh  {
    # server 127.0.0.1:8009;
    # }

    server {
        listen       8009;
        #server_name  127.0.0.1;

        #charset koi8-r;

        #access_log  logs/host.access.log  main;

        location = / {
            root   html;
            index  entry.html index.html index.htm;
            try_files $uri $uri/ /entry.html;
        }

  # 字体文件跨域设置
  location ~* \.(ttf|ttc|otf|eot|woff|woff2|font.css)$ {
    add_header Access-Control-Allow-Origin *;
  }

  # 图片文件跨域设置
  location ~.(png|jpe|jpeg|gif)$ {
    add_header Access-Control-Allow-Origin *;
  }

  # arcgis js api资源
  location ^~ /arcgis/ {
    add_header Access-Control-Allow-Origin *;
    add_header Access-Control-Allow-Credentials 'true';
    add_header Access-Control-Allow-Methods 'GET, POST, OPTIONS';
    add_header Access-Control-Allow-Headers 'DNT,web-token,app-token,Authorization,Accept,Origin,Keep-Alive,User-Agent,X-Mx-ReqToken,X-Data-Type,X-Auth-Token,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type,Range';
    add_header Access-Control-Expose-Headers 'Content-Length,Content-Range';
    if ($request_method = 'OPTIONS') {
      add_header Access-Control-Max-Age 1728000;
      add_header Content-Type 'text/plain; charset=utf-8';
      add_header Content-Length 0;
      return 204;
    }
  }

  # basemap代理(无效)
  location ^~ /maps/vt {
    #add_header Access-Control-Allow-Origin *;
    #add_header Access-Control-Allow-Credentials 'true';
    #add_header Access-Control-Allow-Methods 'GET, POST, OPTIONS';
    #add_header Access-Control-Allow-Headers 'DNT,web-token,app-token,Authorization,Accept,Origin,Keep-Alive,User-Agent,X-Mx-ReqToken,X-Data-Type,X-Auth-Token,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type,Range';
    #add_header Access-Control-Expose-Headers 'Content-Length,Content-Range';
    #proxy_set_header  Host $host;
    #proxy_set_header  X-Real-IP    $remote_addr;
    #proxy_set_header  X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_pass http://www.google.cn/maps/vt/;
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection "upgrade";
  }

  # 5G百宝箱
        location ^~ /bbx/ {
            try_files $uri $uri/ /bbx/index.html;
        }

        location ^~ /bbx/api {
            # rewrite  ^.+bbx/?(.*)$ /$1 break;
            include  uwsgi_params;
            proxy_set_header  Host $host;
            proxy_set_header  X-Real-IP    $remote_addr;
            proxy_set_header  X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_pass http://127.0.0.1:8098/bbx;
            #proxy_cookie_path ~*^ /bbx/;
        }

  #location ^~ /bbx/arcgis {
  #    proxy_pass https://js.arcgis.com/4.15/;
  #}

        # 干线规划
        location ^~ /aosh/ {
            try_files $uri $uri/ /aosh/index.html;
        }

        location ^~ /aosh/api {
            # rewrite  ^.+aosh/?(.*)$ /$1 break;
            include  uwsgi_params;
            proxy_set_header  Host $host;
            proxy_set_header  X-Real-IP    $remote_addr;
            proxy_set_header  X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_pass http://127.0.0.1:9900/aosh;
            #proxy_cookie_path ~*^ /aosh/;
        }

  #location ^~ /aosh/arcgis {
  #    proxy_pass https://js.arcgis.com/4.15/;
  #}

        # 用户中心
        location ^~ /uaa/ {
            try_files $uri $uri/ /uaa/index.html;
        }

        location ^~ /uaa/api {
            include  uwsgi_params;
            proxy_set_header  Host $host;
            proxy_set_header  X-Real-IP    $remote_addr;
            proxy_set_header  X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_pass http://127.0.0.1:9900/uaa;
            #proxy_cookie_path ~*^ /uaa/;
        }

        error_page  404              /404.html;

        # To allow POST on static pages 允许静态页使用POST方法
        # error_page  405     =200 $uri;

        # redirect server error pages to the static page /50x.html
        #
        error_page   500 502 503 504  /50x.html;
        location = /50x.html {
            root   html;
        }

        # proxy the PHP scripts to Apache listening on 127.0.0.1:80
        #
        #location ~ \.php$ {
        #    proxy_pass   http://127.0.0.1;
        #}

        # pass the PHP scripts to FastCGI server listening on 127.0.0.1:9000
        #
        #location ~ \.php$ {
        #    root           html;
        #    fastcgi_pass   127.0.0.1:9000;
        #    fastcgi_index  index.php;
        #    fastcgi_param  SCRIPT_FILENAME  /scripts$fastcgi_script_name;
        #    include        fastcgi_params;
        #}

        # deny access to .htaccess files, if Apache's document root
        # concurs with nginx's one
        #
        #location ~ /\.ht {
        #    deny  all;
        #}
    }

    # another virtual host using mix of IP-, name-, and port-based configuration
    #
    #server {
    #    listen       8000;
    #    listen       somename:8080;
    #    server_name  somename  alias  another.alias;

    #    location / {
    #        root   html;
    #        index  index.html index.htm;
    #    }
    #}


    # HTTPS server
    #
    #server {
    #    listen       443 ssl;
    #    server_name  localhost;

    #    ssl_certificate      cert.pem;
    #    ssl_certificate_key  cert.key;

    #    ssl_session_cache    shared:SSL:1m;
    #    ssl_session_timeout  5m;

    #    ssl_ciphers  HIGH:!aNULL:!MD5;
    #    ssl_prefer_server_ciphers  on;

    #    location / {
    #        root   html;
    #        index  index.html index.htm;
    #    }
    #}

}

```
