![demo](demo.png)
# Rust企业级一站式后台解决方案
*  坚如磐石（Rust语言），高性能，无GC无内存泄漏，无协程竞争
*  DDD领域驱动,Mysql,Redis，通用中间件和数据库，通用企业级框架选型
*  [rbatis-orm](https://github.com/rbatis/rbatis) 和Mybatis-Plus一样的好用，简洁，易扩展的ORM框架
*  [fast_log](https://github.com/rbatis/fast_log) 超快速异步日志框架，支持zip压缩，切割
*  [actix-web](https://actix.rs/) 常年屠榜web框架压测网站的框架
*  前后端分离,基于 [Vue-JS](https://cn.vuejs.org/) +[Vue-AntDesign](https://www.antdv.com/docs/vue/introduce-cn/) + [Vue-AntDesign-Pro](https://pro.antdv.com/)
*  RBAC权限控制,自带JwtToken鉴权登陆，图形验证码登陆，二维码扫码登陆,基础权限管理

# 进度、功能模块（包含(包含web前端和rust后端)）

|  功能(包含web前端和rust后端)    |   完成(√)、进行中(x)     | 
|  ------ | ------ |
| 动态菜单(菜单路由表权限动态生成)  |  √    | 
| JWT账号密码登陆   |  √   |     
| JWT拦截器校验   |  √   |   
| JWT图形验证码登陆   |  x   |     
| JWT二维码扫码登陆   |  x   |  
| JWT短信登陆（基于redis短信消息）   |  x   |  
| 设置/权限管理（父子级，分菜单权限+按钮权限，缓存redis）   |  √   |    
| 设置/角色管理（父子级,分层级权限树，缓存redis）   |  √   | 
| 设置/后台账号管理（分层角色树）   |  √   | 
| 设置/键值对常量管理   |  x   | 
| 设置/键值对常量管理   |  x   | 

# 此项目存在的意义
* 高性能，快如C++, 超低内存占用，支持廉价服务器
* 稳定，部署无忧，无内存泄漏，无闪退
* 开箱即用

# （rust服务器端安装）快速安装教程
* 1.（rust服务器端安装）1.docker命令快速启动redis和mysql(用户名root密码123456)。生产docker可以建议部署http服务，原则上生产环境不建议用docker部署数据库
```cmd
docker run -it -d --name redis -p 6379:6379 redis
docker run -d -p 3306:3306 -e MYSQL_ROOT_PASSWORD=123456 --name mysql -e TZ=Asia/Shanghai mysql:5.7
```
* 2.（rust服务器端安装）使用MysqlWorkBench或Navicat等工具 导入database.sql脚本到Mysql数据库（mysql用户名密码root  123456）（redis无密码）中

# （前端node服务安装）快速安装教程
* 1.（前端安装）阅读并克隆前端项目 https://github.com/rbatis/abs_admin_vue

* 2.（前端安装）使用``` npm install ```安装依赖（或者淘宝镜像cnpm）并使用 ``` yarn serve ```命令启动web前端

* 3.（前端安装）打开浏览器http://localhost:8001 即可登陆后台

# （postman导入）教程
* 1.（postman安装）安装打开PostMan ，导入postman.json到postman中即可使用写好的请求
```cmd
打开postman,导入 postman.json
```
* 2.（postman安装）使用Clion克隆导入abs_admin项目，点开main.rs点击按钮运行.或执行命令:
```cmd
cargo update
cargo run
```


# module(模块)
* JWT token Auth(基于JWT token的权限鉴权)
* Role,User,Reource（角色，用户，权限）


