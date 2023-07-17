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
    include /etc/nginx/conf.d/*.conf;
    default_type  application/octet-stream;

    resolver 8.8.8.8;

    #log_format  main  '$remote_addr - $remote_user [$time_local] "$request" '
    #                  '$status $body_bytes_sent "$http_referer" '
    #                  '"$http_user_agent" "$http_x_forwarded_for"';

    #access_log  logs/access.log  main;

    sendfile            on;
    tcp_nopush          on;
    tcp_nodelay         on;
    types_hash_max_size 2048;
    keepalive_timeout  300;
    send_timeout 300;
    client_max_body_size 300m;
    
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

    # 取消nginx对header中包含下划线字段的默认过滤
    underscores_in_headers on;

    gzip  on;

    # upstream documentQuery  {
    #     server https://20.26.85.26:6067/api/v1/;
    # }
    
    # 陕西交投项目配置
    server {
        listen 8082;
        server_name 127.0.0.1;
        
        root /opt/homebrew/var/www;
        
        location = / {
            rewrite ^.+ http://127.0.0.1:8082/shanxi_jiaotou break;
        }
        
        location ^~ /shanxi_jiaotou {
            try_files $uri $uri/ /shanxi_jiaotou/index.html;
        }
        
        location ^~ /shanxi_jiaotou/api {
            include  uwsgi_params;
            proxy_set_header  Host $host;
            proxy_set_header  X-Real-IP    $remote_addr;
            proxy_set_header  X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
            proxy_pass_request_headers on;
            proxy_pass http://192.168.1.101:8098/;
        }
        
        # 图片文件跨域设置
        location ~.(png|jpe|jpeg|gif)$ {
            add_header Access-Control-Allow-Origin *;
        }
        
        error_page  404              /404.html;

        # 405处理 To allow POST on static pages 允许静态页使用POST方法
        # error_page  405     =200 $uri;

        # redirect server error pages to the static page /50x.html
        error_page   500 502 503 504  /50x.html;
        location = /50x.html {
            root   /opt/homebrew/var/www/shanxi_jiaotou;
        }
    }
    

    server {
        listen 80;
        server_name  127.0.0.1;

        # 自动跳转到 HTTPS
        rewrite ^(.*)$ https://$host$1 permanent;

        # 使用return的效率会更高
        # return 301 https://$server_name$request_uri;

        # 让http请求重定向到https请求
        error_page 497  https://$host$uri?$args;
    }

    server {
        listen 443 ssl;
        listen [::]:443 ssl;
        server_name  127.0.0.1;

        fastcgi_param   HTTPS               on;
        fastcgi_param   HTTP_SCHEME         https;

        # ssl证书的pem文件路径
        ssl_certificate  cert/server.crt;
        # ssl证书的key文件路径
        ssl_certificate_key cert/server.key;

        # 开启双向验证，浏览器未安装证书会报 400
        # 浏览器安装p12格式证书
        ssl_verify_client on;
        ssl_client_certificate cert/ca.crt;

        ssl_session_cache    shared:SSL:1m;
        ssl_session_timeout  5m;
        ssl_protocols TLSv1 TLSv1.1 TLSv1.2; 
        ssl_ciphers ECDH:AESGCM:HIGH:!RC4:!DH:!MD5:!3DES:!aNULL:!eNULL;
        ssl_prefer_server_ciphers  on;

        location / {
            root   html;
            index  documentQuery/entry.html documentQuery/index.html documentQuery/index.htm;
            try_files $uri $uri/ /documentQuery/index.html;
        }

        location ^~ /documentQuery/ {
            try_files $uri $uri/ /documentQuery/index.html;
        }

        location ^~ /documentQuery/api/v1 {
            include  uwsgi_params;
            proxy_pass_request_headers on;
            proxy_set_header  Host $host;
            proxy_set_header  X-Real-IP $remote_addr;
            proxy_set_header  X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header  X-Forwarded-Proto $scheme;

            proxy_pass https://20.26.85.26:6067/api/v1;
            # ssl证书的pem文件路径
            proxy_ssl_certificate  cert/client.crt;
            # ssl证书的key文件路径
            proxy_ssl_certificate_key cert/client.key;

            proxy_ssl_protocols TLSv1 TLSv1.1 TLSv1.2;
            proxy_ssl_ciphers ECDH:AESGCM:HIGH:!RC4:!DH:!MD5:!3DES:!aNULL:!eNULL;
            proxy_ssl_session_reuse on;
            proxy_ssl_verify_depth 2;
            proxy_next_upstream http_500 http_502 http_503 http_504 timeout error invalid_header;
            # proxy_cookie_path ~*^ /documentQuery/;
        }

        error_page  404              /404.html;

        # 让http请求重定向到https请求
        error_page 497  https://$host$uri?$args;

        # To allow POST on static pages 允许静态页使用POST方法
        error_page  405     =200 $uri;

        # redirect server error pages to the static page /50x.html
        error_page   500 502 503 504  /50x.html;
        location = /50x.html {
            root   html;
        }
    }

    server {
        listen       8009;
        server_name  127.0.0.1;

        #charset koi8-r;

        #access_log  logs/host.access.log  main;

        location = / {
            root   html;
            index  bbx/entry.html bbx/index.html bbx/index.htm;
            try_files $uri $uri/ /bbx/index.html;
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

          # basemap图片资源代理
          location ^~ /appmaptile {
            add_header Access-Control-Allow-Origin *;
            add_header Access-Control-Allow-Credentials 'true';
            add_header Access-Control-Allow-Methods 'GET, POST, OPTIONS';
            add_header Access-Control-Allow-Headers 'DNT,web-token,app-token,Authorization,Accept,Origin,Keep-Alive,User-Agent,X-Mx-ReqToken,X-Data-Type,X-Auth-Token,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type,Range';
            add_header Access-Control-Expose-Headers 'Content-Length,Content-Range';
            proxy_pass http://webst01.is.autonavi.com/appmaptile$request_uri;
            proxy_set_header Upgrade $http_upgrade;
            proxy_set_header Connection "upgrade";
          }

          # 5G百宝箱
        location ^~ / {
            try_files $uri $uri/ /bbx/index.html;
        }

        location ^~ /api/ {
            # rewrite  ^.+bbx/?(.*)$ /$1 break;
            include  uwsgi_params;
            proxy_set_header  Host $host;
            proxy_set_header  X-Real-IP    $remote_addr;
            proxy_set_header  X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_pass http://192.168.1.101:8098/;
            #proxy_cookie_path ~*^ /bbx/;
        }

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
            # proxy_pass http://218.205.107.96:9900/aosh;
            proxy_pass http://127.0.0.1:9900/aosh;
            #proxy_cookie_path ~*^ /aosh/;
        }

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

        # 成果管理系统
        location ^~ /adms/ {
            try_files $uri $uri/ /adms/index.html;
        }

        location ^~ /adms/api/ {
            include  uwsgi_params;
            proxy_set_header  Host $host;
            proxy_set_header  X-Real-IP    $remote_addr;
            proxy_set_header  X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_pass http://192.168.1.209:9954/adms/;
            #proxy_cookie_path ~*^ /adms/;
        }

        # wrr
        # location ^~ /wrr/ {
        #     try_files $uri $uri/ /wrr/index.html;
        # }

        # location ^~ /wrr/api/ {
        #     include  uwsgi_params;
        #     proxy_set_header  Host $host;
        #     proxy_set_header  X-Real-IP    $remote_addr;
        #     proxy_set_header  X-Forwarded-For $proxy_add_x_forwarded_for;
        #     proxy_pass http://192.168.1.220:9954/wrr/;
        #     #proxy_cookie_path ~*^ /wrr/;
        # }

        error_page  404              /404.html;

        # 405 处理
        error_page  405     =200 $uri;

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

    server {
        # listen       8011;

        #charset koi8-r;

        #access_log  logs/host.access.log  main;

        # root html;
        # index documentQuery/index.html;

        # rewrite ^(.*)$ https://$host$1 permanent;

        # location = / {
        #     root   html;
        #     index  documentQuery/entry.html documentQuery/index.html documentQuery/index.htm;
        #     try_files $uri $uri/ /documentQuery/index.html;
        # }
  
        # location ^~ / {
        #     try_files $uri $uri/ /documentQuery/index.html;
        # }

        # location ^~ /documentQuery/ {
        #     try_files $uri $uri/ /documentQuery/index.html;
        # }

        # location ^~ /documentQuery/api/v1 {
        #     include  uwsgi_params;
        #     proxy_set_header  Host $host;
        #     proxy_set_header  X-Real-IP    $remote_addr;
        #     proxy_set_header  X-Forwarded-For $proxy_add_x_forwarded_for;
        #     proxy_pass https://20.26.85.26:6067/documentQuery;
        #     #proxy_cookie_path ~*^ /documentQuery/;
        # }

        error_page  404              /404.html;

        # 405 处理
        error_page  405     =200 $uri;

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
