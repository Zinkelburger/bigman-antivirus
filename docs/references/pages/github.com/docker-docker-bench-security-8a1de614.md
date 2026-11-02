---
url: "https://github.com/docker/docker-bench-security"
title: "GitHub - docker/docker-bench-security: The Docker Bench for Security is a script that checks for dozens of common best-practices around deploying Docker containers in production."
site: "GitHub"
captured: "2026-11-02T18:51:26Z"
via: "live"
sha256: "265f69e68d3eb1eeee3d99deebfd9975d58e70d69d18730b99cae338c9237e14"
wayback: "https://web.archive.org/web/20261022212640/https://github.com/docker/docker-bench-security"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# GitHub - docker/docker-bench-security: The Docker Bench for Security is a script that checks for dozens of common best-practices around deploying Docker containers in production.

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/docker/docker-bench-security>
> Snapshot: <https://web.archive.org/web/20261022212640/https://github.com/docker/docker-bench-security>

# Docker Bench for Security

https://github.com/docker/docker-bench-security#docker-bench-for-security

https://github.com/docker/docker-bench-security/blob/master/img/benchmark_log.png

The Docker Bench for Security is a script that checks for dozens of common best-practices around deploying Docker containers in production. The tests are all automated, and are based on the [CIS Docker Benchmark v1.6.0](https://www.cisecurity.org/benchmark/docker/).

We are making this available as an open-source utility so the Docker community can have an easy way to self-assess their hosts and Docker containers against this benchmark.

| Release  | CIS

| 1.6.0  | 1.6.0

| 1.5.0  | 1.5.0

| 1.3.6  | 1.4.0

| 1.3.5  | 1.2.0

| 1.3.3  | 1.1.0

| 1.3.0  | 1.13.0

## Running Docker Bench for Security

https://github.com/docker/docker-bench-security#running-docker-bench-for-security

### Run from your base host

https://github.com/docker/docker-bench-security#run-from-your-base-host

You can simply run this script from your base host by running:

```
git clone https://github.com/docker/docker-bench-security.git
cd docker-bench-security
sudo sh docker-bench-security.sh
```

Note: [jq](https://jqlang.github.io/jq/) is an optional but recommended dependency.

### Run with Docker

https://github.com/docker/docker-bench-security#run-with-docker

#### Building Docker image

https://github.com/docker/docker-bench-security#building-docker-image

You have two options if you wish to build and run this container yourself:

- Use Docker Build:

```
git clone https://github.com/docker/docker-bench-security.git
cd docker-bench-security
docker build --no-cache -t docker-bench-security .
```

Followed by an appropriate docker run command as stated below.

- Use Docker Compose:

```
git clone https://github.com/docker/docker-bench-security.git
cd docker-bench-security
docker-compose run --rm docker-bench-security
```

Please note that the docker/docker-bench-security image is out-of-date and and a manual build is required. See [#405](https://github.com/docker/docker-bench-security/issues/405) for more information.

Note that this container is being run with a lot of privilege -- sharing the host's filesystem, pid and network namespaces, due to portions of the benchmark applying to the running host.

### Using the container

https://github.com/docker/docker-bench-security#using-the-container

```
docker run --rm --net host --pid host --userns host --cap-add audit_control \
    -e DOCKER_CONTENT_TRUST=$DOCKER_CONTENT_TRUST \
    -v /etc:/etc:ro \
    -v /usr/bin/containerd:/usr/bin/containerd:ro \
    -v /usr/bin/runc:/usr/bin/runc:ro \
    -v /usr/lib/systemd:/usr/lib/systemd:ro \
    -v /var/lib:/var/lib:ro \
    -v /var/run/docker.sock:/var/run/docker.sock:ro \
    --label docker_bench_security \
    docker-bench-security
```

Don't forget to adjust the shared volumes according to your operating system. Some examples are:

- On Ubuntu the docker.service and docker.secret files are located in /lib/systemd/system folder by default.

```
docker run --rm --net host --pid host --userns host --cap-add audit_control \
    -e DOCKER_CONTENT_TRUST=$DOCKER_CONTENT_TRUST \
    -v /etc:/etc:ro \
    -v /lib/systemd/system:/lib/systemd/system:ro \
    -v /usr/bin/containerd:/usr/bin/containerd:ro \
    -v /usr/bin/runc:/usr/bin/runc:ro \
    -v /usr/lib/systemd:/usr/lib/systemd:ro \
    -v /var/lib:/var/lib:ro \
    -v /var/run/docker.sock:/var/run/docker.sock:ro \
    --label docker_bench_security \
    docker-bench-security
```

- The /etc/hostname file is missing on macOS, so it will need to be created first. Also, Docker Desktop on macOS doesn't have /usr/lib/systemd or the above Docker binaries.

```
sudo touch /etc/hostname

docker run --rm --net host --pid host --userns host --cap-add audit_control \
    -e DOCKER_CONTENT_TRUST=$DOCKER_CONTENT_TRUST \
    -v /etc:/etc \
    -v /var/lib:/var/lib:ro \
    -v /var/run/docker.sock:/var/run/docker.sock:ro \
    --label docker_bench_security \
    docker-bench-security
```

### Note

https://github.com/docker/docker-bench-security#note

Docker bench requires Docker 1.13.0 or later in order to run.

Note that when distributions don't contain auditctl, the audit tests will check /etc/audit/audit.rules to see if a rule is present instead.

### Docker Bench for Security options

https://github.com/docker/docker-bench-security#docker-bench-for-security-options

```
  -b           optional  Do not print colors
  -h           optional  Print this help message
  -l FILE      optional  Log output in FILE, inside container if run using docker
  -u USERS     optional  Comma delimited list of trusted docker user(s)
  -c CHECK     optional  Comma delimited list of specific check(s) id
  -e CHECK     optional  Comma delimited list of specific check(s) id to exclude
  -i INCLUDE   optional  Comma delimited list of patterns within a container or image name to check
  -x EXCLUDE   optional  Comma delimited list of patterns within a container or image name to exclude from check
  -t LABEL     optional  Comma delimited list of labels within a container or image to check
  -n LIMIT     optional  In JSON output, when reporting lists of items (containers, images, etc.), limit the number of reported items to LIMIT. Default 0 (no limit).
  -p PRINT     optional  Disable the printing of remediation measures. Default: print remediation measures.
```

By default the Docker Bench for Security script will run all available CIS tests and produce logs in the log folder from current directory, named docker-bench-security.log.json and docker-bench-security.log.

If the docker container is used then the log files will be created inside the container in location /usr/local/bin/log/. If you wish to access them from the host after the container has been run you will need to mount a volume for storing them in.

The CIS based checks are named check_<section>_<number>, e.g. check_2_6 and community contributed checks are named check_c_<number>.

sh docker-bench-security.sh -c check_2_2 will only run check 2.2 Ensure the logging level is set to 'info'.

sh docker-bench-security.sh -e check_2_2 will run all available checks except 2.2 Ensure the logging level is set to 'info'.

sh docker-bench-security.sh -e docker_enterprise_configuration will run all available checks except the docker_enterprise_configuration group

sh docker-bench-security.sh -e docker_enterprise_configuration,check_2_2 will run all available checks except the docker_enterprise_configuration group and 2.2 Ensure the logging level is set to 'info'

sh docker-bench-security.sh -c container_images,container_runtime will run just the container_images and container_runtime checks

sh docker-bench-security.sh -c container_images -e check_4_5 will run just the container_images checks except 4.5 Ensure Content trust for Docker is Enabled

Note that when submitting checks, provide information why it is a reasonable test to add and please include some kind of official documentation verifying that information.
