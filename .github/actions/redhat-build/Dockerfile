# Container image that runs your code
FROM redhat/ubi8:8.10

RUN dnf install gcc -y && \
    dnf install gcc-c++ -y && \
    dnf install make -y && \
    dnf install cmake -y && \
    dnf install git -y && \
    dnf install python3 -y && \
    dnf install python3-pip -y && \
    dnf install python3-setuptools -y && \
    dnf install python3-wheel -y && \
    dnf install python3-devel -y && \
    dnf install libstdc++-devel -y && \
    dnf install llvm-toolset -y

# Copies your code file from your action repository to the filesystem path `/` of the container
COPY entrypoint.sh /entrypoint.sh


# Code file to execute when the docker container starts up (`entrypoint.sh`)
ENTRYPOINT ["/entrypoint.sh"]
