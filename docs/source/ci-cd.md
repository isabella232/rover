---
title: "Using Rover in CI/CD"
sidebar_title: "CI/CD"
---

You can use Rover in any CI/CD environment that uses a Rover-supported operating system (Linux, MacOS, or Windows). Most commonly, this is to run [schema checks](https://www.apollographql.com/docs/studio/schema-checks/) with [`rover graph check`](./graphs/#checking-schema-changes) or [`rover subgraph check`](./subgraphs/#checking-subgraph-schema-changes).

Rover's installation is similar to many other CLI tools, but the recommended method varies depending on which provider you're using. We've included instructions for some of the most common CI/CD providers:

* [CircleCI](#circleci)
* [GitHub Actions](#github-actions)
* [Bitbucket Pipelines](#bitbucket-pipelines).


> If you're using Rover with a CI/CD provider not listed here, we'd love for you to share the steps by opening an [issue](https://github.com/apollographql/rover/issues/new/choose) or [pull request](https://github.com/apollographql/rover/compare)!

## CircleCI

### Linux jobs using the `curl` installer

Normally when installing, Rover adds the path of its executable to your `$PATH`. However, CircleCI doesn't use the `$PATH` variable between run `step`s. This means that if you install Rover and try to run it in the next step, you get a `command not found: rover` error.

To fix this, you can modify the `$PATH` and append it to [`$BASH_ENV`](https://circleci.com/docs/2.0/env-vars/#setting-an-environment-variable-in-a-shell-command). `$BASH_ENV` is executed at the beginning of each step, enabling any changes to be maintained across steps. You can add Rover to your `$PATH` using `$BASH_ENV` like this:

```bash
echo 'export PATH=$HOME/.rover/bin:$PATH' >> $BASH_ENV
```

After you install Rover and modify the `$BASH_ENV` as shown, Rover should work like normal.

> **Important:** Because the `rover config auth` command is interactive, you need to [authenticate using an environment variable](./configuring#with-an-environment-variable) in your project settings.

#### Full example

```yaml
# Use the latest 2.1 version of CircleCI pipeline process engine. See: https://circleci.com/docs/2.0/configuration-reference
version: 2.1

jobs:
  build:
    docker:
      - image: cimg/node:15.11.0
    steps:
      - run:
          name: Install
          command: |
            # download and install Rover
            curl -sSL https://rover.apollo.dev/nix/v0.1.0 | sh

            # This allows the PATH changes to persist to the next `run` step
            echo 'export PATH=$HOME/.rover/bin:$PATH' >> $BASH_ENV
      - checkout
      # after rover is installed, you can run it just like you would locally!
      - run: rover graph check my-graph@prod --schema ./schema.graphql
```

## GitHub Actions

### Displaying schema check results on GitHub pull requests

If you use GitHub Actions to automatically run [schema checks](https://www.apollographql.com/docs/studio/schema-checks/) on every pull request ([as shown below](#full-example)), you can install the [Apollo Studio GitHub app](https://github.com/marketplace/apollo-studio) to provide links to the results of those checks alongside your other pull request checks:

<img class="screenshot" src="./assets/checks-result.jpg" width="550"/>

For these entries to display correctly, you need to make sure Rover associates the schema check execution with the pull request's `HEAD` commit, as opposed to the _merge_ commit that GitHub adds. To guarantee this, set the `APOLLO_VCS_COMMIT` environment variable in your action's configuration, like so:

```yaml
env:
  APOLLO_VCS_COMMIT: ${{ github.event.pull_request.head.sha }}
```

### Linux/MacOS jobs using the `curl` installer

Normally when installing, Rover adds the path of its executable to your `$PATH`. However, GitHub Actions doesn't use the `$PATH` variable between run `step`s. This means that if you install Rover and try to run it in the next step, you get a `command not found: rover` error.

To fix this, you can append Rover's location to the [`$GITHUB_PATH`](https://docs.github.com/en/actions/reference/workflow-commands-for-github-actions#adding-a-system-path) variable. `$GITHUB_PATH` is similar to your system's `$PATH` variable, and additions to `$GITHUB_PATH` can be used across multiple steps. You can modify it like this:

```bash
echo "$HOME/.rover/bin" >> $GITHUB_PATH
```

> **Important:** Because the `rover config auth` command is interactive, you need to [authenticate using an environment variable](./configuring#with-an-environment-variable) in your project settings.
>
> GitHub actions uses [project environments](https://docs.github.com/en/actions/reference/environments) to set up secret environment variables. In your action, you choose a `build.environment` by name and set `build.env` variables using the saved secrets.

The following is a full example script, showing how to choose an `apollo` environment and set an `APOLLO_KEY` variable:


#### Full example

```yaml
# .github/workflows/check.yml

name: Check Schema

# Controls when the action will run. Triggers the workflow on push or pull request events
on: [push, pull_request]

# A workflow run is made up of one or more jobs that can run sequentially or in parallel
jobs:
  # This workflow contains a single job called "build"
  build:
    # The type of runner that the job will run on
    runs-on: ubuntu-latest

    # https://docs.github.com/en/actions/reference/environments
    environment: apollo

    # https://docs.github.com/en/actions/reference/encrypted-secrets
    # https://docs.github.com/en/actions/reference/workflow-syntax-for-github-actions#jobsjob_idstepsenv
    env:
      APOLLO_KEY: ${{ secrets.APOLLO_KEY }}
      APOLLO_VCS_COMMIT: ${{ github.event.pull_request.head.sha }}

    # Steps represent a sequence of tasks that will be executed as part of the job
    steps:
      # Checks-out your repository under $GITHUB_WORKSPACE, so your job can access it
      - uses: actions/checkout@v2

      - name: Install Rover
        run: |
          curl -sSL https://rover.apollo.dev/nix/v0.1.0 | sh

          # Add Rover to the $GITHUB_PATH so it can be used in another step
          # https://docs.github.com/en/actions/reference/workflow-commands-for-github-actions#adding-a-system-path
          echo "$HOME/.rover/bin" >> $GITHUB_PATH
      - name: Run check against prod
        run: |
          rover graph check my-graph@prod --schema ./test.graphql

```

## Bitbucket Pipelines

The following is a full example configuration for Bitbucket Pipelines. It shows how to:

* Run `rover subgraph check` for each commit on all branches
* Run `rover subgraph publish` to keep the schema definition of your `main` branch in-sync with a base variant (`@local` in this case)

The example uses the following Pipeline Repository Variables to make the pipeline configuration portable across different repositories:

*  `APOLLO_KEY`
*  `APOLLO_SUBGRAPH_NAME`, which represents the name of the subgraph you're running schema checks for
*  `APOLLO_LOCAL_PORT`, which represents the port number of the base variant

#### Full example

```yaml
# ./bitbucket-pipelines.yml

image: node

definitions:
  steps:
    - step: &rover-subgraph-check
        name: "[Rover] Subgraph Check"
        caches:
          - node
        script:
          - 'echo "Subgraph name: $APOLLO_SUBGRAPH_NAME"'
          - npx -p @apollo/rover@latest
            rover subgraph check my-graph@prod
            --name $APOLLO_SUBGRAPH_NAME
            --schema ./schema.graphql

    - step: &local-publish
        name: "[Rover] @local publish (sync with main/master)"
        caches:
          - node
        script:
          - 'echo "Subgraph name: $APOLLO_SUBGRAPH_NAME"'
          - 'echo "Local variant port: $APOLLO_LOCAL_PORT"'

          - npx -p @apollo/rover@latest
            rover subgraph publish my-graph@local
            --name $APOLLO_SUBGRAPH_NAME
            --schema ./schema.graphql
            --routing-url http://localhost:$APOLLO_LOCAL_PORT/graphql

pipelines:
  default:
    - step: *rover-subgraph-check

  branches:
    '{main,master}':
      - step: *rover-subgraph-check
      - step: *local-publish
```


## Using With `npm`/`npx`

If you're running in a Node.js workflow, it might be easier to use the [NPM distribution of Rover](https://www.npmjs.com/package/@apollo/rover). This way, you don't need to adjust the PATH at all to run Rover, and it might fit better into your existing workflow.

You can use Rover by adding it to your `package.json` dependencies using [these instructions](./getting-started#npm-installer) and then execute it using npm scripts, similar to other workflows you might already have. If you don't want to install Rover as a dependency, you can run it with `npx` by using the `-p` flag:

```bash
npx -p @apollo/rover rover graph check my-graph@prod --schema=./schema.graphql
```

Since most commands require you be authenticated, see the above sections for instructions on how to add environment variables for your CI/CD provider.
