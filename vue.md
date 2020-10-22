---
Author: Virgil-N
Date: 2020-10-22T19:52:58+08:00
---

#

## VUE

- 使用 `typescript` 和 `vue-property-decorator` 并采用 `watch` 监听对象时，在组件中必须使用 `get(computed方法)` 先获取下数据，否则监听方法会报 `undefined` 错误
