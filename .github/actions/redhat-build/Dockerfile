# Container image that runs your code
FROM redhat/ubi8:8.10

RUN dnf install gcc-8.5.0-26.el8_10.x86_64

RUN dnf install llvm-toolset

# Copies your code file from your action repository to the filesystem path `/` of the container
COPY entrypoint.sh /entrypoint.sh


# Code file to execute when the docker container starts up (`entrypoint.sh`)
ENTRYPOINT ["/entrypoint.sh"]
