---
Author: Virgil-N
Date: 2020-08-04T19:52:58+08:00
---
#

## KEEP IN MIND

- GET: curl -i localhost:9292/user/get_by_name/Tom

- GET: query: curl -i "localhost:9292/user/list?name=Ann&current=1&size=10"

- POST: query: curl -i -d "{\"data\":{\"name\":\"Tom\"},\"size\":5}" -H 'Content-Type:application/json' localhost:9292/user/list

- POST json数据: curl -i -d "{\"name\":\"Tom\"}" -H 'Content-Type:application/json' localhost:9292/user/add

- DELETE: curl -i -X DELETE localhost:9292/user/delete/2wewewe
