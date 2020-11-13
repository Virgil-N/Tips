#

## proto

### grpc

- 命令: protoc -I . --go_out ./  --go_opt paths=source_relative --go-grpc_out ./ --go-grpc_opt paths=source_relative your_service.proto

### grpcweb

- 命令: protoc --plugin=C:\mySoftware\grpc-web\protoc-gen-grpc-web.exe --js_out=import_style=commonjs:./ user.proto --grpc-web_out=import_style=commonjs+dts,mode=grpcwebtext:./

### grpcwebproxy

- 命令: --backend_addr=localhost:50051 --backend_tls_noverify --run_tls_server=false --allow_all_origins
- 跨域设置可以指定允许的地址: --allowed_origins=https://example.org,https://awesome.com

### grpc-gateway

- 命令: protoc -I . --grpc-gateway_out ./ --grpc-gateway_opt logtostderr=true --grpc-gateway_opt paths=source_relative --grpc-gateway_opt generate_unbound_methods=true your_service.proto