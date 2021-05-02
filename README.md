# kaniko-compose-builder
simple wrapper around [kaniko](https://github.com/GoogleCloudPlatform/kaniko) to build from docker-compose files

## Example use from gitlab-ci:

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
