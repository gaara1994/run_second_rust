# 使用基于 Alpine 的镜像作为基础镜像
FROM rust:latest

# 将你的可执行文件复制到镜像中
COPY ./target/release/run_second_rust /usr/local/bin/

# 设置默认命令为运行你的可执行文件
CMD ["run_second_rust"]