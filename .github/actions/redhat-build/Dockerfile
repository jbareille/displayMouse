# Container image that runs your code
FROM redhat/ubi8:8.10

RUN cat /etc/redhat-release

RUN dnf provides '*bin/cc'

RUN dnf group install "Development Tools"

RUN dnf install llvm-toolset

# Copies your code file from your action repository to the filesystem path `/` of the container
COPY entrypoint.sh /entrypoint.sh


# Code file to execute when the docker container starts up (`entrypoint.sh`)
ENTRYPOINT ["/entrypoint.sh"]
