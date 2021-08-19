#

## HTTPS 证书的配置

- 首先自己生成证书需要安装 openssl 工具

- 由于是使用openssl架设私有证书中心，因此要保证以下字段在证书中心的证书、服务端证书、客户端证书中都相同:

```
Country Name
State or Province Name
Locality Name
Organization Name
Organizational Unit Name
```

### 编辑证书中心配置文件

```
vim /etc/pki/tls/openssl.cnf
[ CA_default ]
 dir             = /etc/pki/CA
 certs           = $dir/certs            # Where the issued certs are kept
 crl_dir         = $dir/crl              # Where the issued crl are kept
 database        = $dir/index.txt        # database index file.
 #unique_subject = no                    # Set to 'no' to allow creation of
 # several ctificates with same subject.
 new_certs_dir   = $dir/newcerts         # default place for new certs.
 certificate     = $dir/cacert.pem       # The CA certificate
 serial          = $dir/serial           # The current serial number
 crlnumber       = $dir/crlnumber        # the current crl number                                        # must be commented out to leave a V1 CRL
 crl             = $dir/crl.pem          # The current CRL
 private_key     = $dir/private/cakey.pem# The private key
 RANDFILE        = $dir/private/.rand    # private random number file
[ req_distinguished_name ]
 countryName                     = Country Name(2 letter code)
 countryName_default             = CN
 countryName_min                 = 2
 countryName_max                 = 2
 stateOrProvinceName             = State or Province Name (full name)
 stateOrProvinceName_default     = FJ
 localityName                    = Locality Name (eg, city)
 localityName_default            = FZ
 0.organizationName              = Organization Name (eg, company)
 0.organizationName_default      = zdz
 organizationalUnitName          = Organizational Unit Name (eg, section)
 organizationalUnitName_default  = zdz

```

- 生成CA私钥:

```
openssl genrsa -out local.key 2048
```

- 生成CA证书请求

```
openssl req -new -key local.key -out local.csr
// Country Name (2 letter code) [XX]:CN  #国家
// State or Province Name (full name) []:BJ   #省份
// Locality Name (eg, city) [Default City]:BJ  #城市
// Organization Name (eg, company) [Default Company Ltd]:
// Organizational Unit Name (eg, section) []:test   #部门
// Common Name (eg, your name or your server's hostname) []:test   #主机名
// Email Address []:test@test.com  #邮箱 
// Please enter the following 'extra' attributes
// to be sent with your certificate request
// A challenge password []:wuminyan  #密码
// An optional company name []:wuminyan  #姓名
```

- 生成CA根证书

```
openssl x509 -req -in local.csr -extensions v3_ca -signkey local.key -out local.crt
```

### 根据CA证书创建server端证书

- 生成server私匙

```
openssl genrsa -out my_server.key 2048
```

- 生成server证书请求

```
openssl x509 -req -in local.csr -extensions v3_ca -signkey local.key -out local.crt
openssl genrsa -out my_server.key 2048
openssl req -new -key my_server.key -out my_server.csr
```

- 生成server证书

```
openssl x509 -days 365 -req -in my_server.csr -extensions v3_req -CAkey local.key -CA local.crt -CAcreateserial -out my_server.crt
```

### 根据CA证书创建client端证书

- 和server端证书一样

### 根据client端证书生成可导入浏览器的证书

```
// 将文本格式的证书转换成可以导入浏览器的证书
openssl pkcs12 -export -clcerts -in client.crt -inkey client.key -out client.p12
```

### 配置nginx服务器验证

详见nginx.md

```
vim /usr/local/nginx/conf/nginx.conf
ssl on;
ssl_certificate         /usr/local/nginx/ssl/nginx.crt;
ssl_certificate_key     /usr/local/nginx/ssl/nginx.key;
ssl_client_certificate  /usr/local/nginx/ssl/cacert.crt;
ssl_session_timeout     5m;
#ssl_verify_client      on; 服务器验证客户端，暂时不开启，让没有证书的客户端可以访问，先完成单向验证
ssl_protocols           SSLv2 SSLv3 TLSv1;

```
