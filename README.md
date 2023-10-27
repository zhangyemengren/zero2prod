## 环境信息
- docker
  - 安装docker desktop
  - 下载postgres官方镜像
- sqlx cli
  - cargo install sqlx-cli --no-default-features --features native-tls,postgres
## scripts 文件夹下的脚本

### init_db.sh
启动 docker 数据库 postgresDB
镜像为官方镜像 postgres

### curl
curl -i -X POST -d 'email=thomas_mann2@hotmail.com&name=Tom2' \
http://127.0.0.1:8000/subscriptions

curl http://127.0.0.1:8000/health_check

### 构建docker镜像
docker build --tag zero2prod --file Dockerfile --platform linux/amd64 .
#### 查看镜像信息
docker images zero2prod
#### run
docker run -p 8000:8000 zero2prod
