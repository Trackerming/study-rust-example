FROM amazonlinux:2023
WORKDIR /app
# 安装node
RUN yum install https://rpm.nodesource.com/pub_16.x/nodistro/repo/nodesource-release-nodistro-1.noarch.rpm -y
RUN  yum install nodejs -y --setopt=nodesource-nodejs.module_hotfixes=1
# 启动文件
COPY ./run.sh /app/run.sh
RUN chmod +x /app/run.sh
COPY ["package*.json","yarn.lock","./"]
COPY ["./enclave.ts", "/app/"]
COPY ./vsock-node /app/
EXPOSE 8000
CMD ["/app/run.sh"]