# kaniko-compose-builder
simple wrapper around [kaniko](https://github.com/GoogleCloudPlatform/kaniko) to build from docker-compose files

## Example use from commandline
(not usefull by it self, as one just could use docker-compose build, if one can execute docker)
```
docker run  --volume $PWD:/workspace thejonny/kaniko-compose-builder kaniko-compose-builder --force
```

## Example use from gitlab-ci:
(where running another docker inside docker is not possible in most cases)

```
#.gitlab-ci.yml

stages:
  - build

build_job:
  stage: build
  # build and push images from the docker-compose file
  image: "thejonny/kaniko-compose-builder"
  script: kaniko-compose-builder
  artifacts:
    paths:
      - docker-compose.yaml
  tags:
    - docker
```

```
# docker-compose.yaml

version: "3"

services:
  hellod:
    # as a build property exists, this one is built and pushed
    image: some.redacted.registry:5000/hellod
    build: hellod
    ports:
        - "1234:1234/udp"
  db:
    # an example dependency, not used at all
    restart: unless-stopped
    image: redis
```
