# Changelog

All notable changes to Rover will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- # [x.x.x] (unreleased) - 2022-mm-dd

> Important: X breaking changes below, indicated by **❗ BREAKING ❗**

## ❗ BREAKING ❗

## 🚀 Features

## 🐛 Fixes

## 🛠 Maintenance

## 📚 Documentation -->

# [0.4.3] - 2022-02-14

## 🐛 Fixes

- **Fix npm installer logic for detecting glibc compatibility - @EverlastingBugstopper, #1006 fixes #1004**

  Our npm installer had a dependency that updated to v2.0 which included some breaking changes. These changes made our npm installer always return false when checking if the operating system has an appropriate version of `glibc` installed (in order to download the version of Rover with `supergraph compose` functionality). We have now moved to the appropriate API and installs of v0.4.3 should work as expected.

# [0.4.2] - 2022-02-11

## 🚀 Features

- **Adds launch URL feedback to `rover subgraph publish` - @Y-Guo, #989**

  If a `rover subgraph publish` invocation kicks off a [launch](https://www.apollographql.com/docs/studio/launches/), it will now output the URL associated with the launch so you can go right to it in Studio.

- **Improve messaging for `rover subgraph check` - @david-castaneda, #980**

  In the case where the API schema does not change after a `subgraph check` but the core schema was modified, you are now notified as such rather than being displayed a confusing "there were no changes detected" message.

## 🐛 Fixes

- **Omit Float scalars from introspection result - @lrlna, #988 fixes #985**

  Since `Float` is a built-in GraphQL scalar type, it is no longer included in Rover's introspection results.

- **Fix configuration error messages when there are no profiles - @EverlastingBugstopper, #995 fixes #783**

  Rover had a hard time detecting when there were no configured profiles, so it never actually emitted error `E020`. Now it does, and we also get some performance improvements by only parsing environment variables on startup rather than on every time they are needed.

- **No longer panic when printing large output - @EverlastingBugstopper, #955 fixes #842**

  Migrate away from the `println` and `eprintln` macros that can cause panics from time to time and instead use calm_io to gracefully handle output.

- **Lowers the maximum required `glibc` version from 2.18 to 2.17 - @EverlastingBugstopper, #990 fixes #991**

  We build Rover in Docker with the `centos:7` base image, which has `glibc` `v2.17` installed. This means we can lower our installer requirements to 2.17, allowing folks on older machines like centos to install Rover. Thanks to @theJC for the report and help with testing!

- **Removes unused `--profile` argument from `rover subgraph introspect` - @EverlastingBugstopper, #950 fixes #948**

## 🛠 Maintenance

- **Use `apollo-encoder` in `rover-client` - @lrlna, #939**

  `sdl-encoder` has been replaced with the official `apollo-encoder` from the [`apollo-rs`](https://github.com/apollographql/apollo-rs) ecosystem.

- **Add PR template - @EverlastingBugstopper, #488**

  Now folks who are kind enough to contribute pull requests to Rover's repository will have a helpful template to fill out.

- **Fix mechanism for building old versions of Rover - @EverlastingBugstopper, #973 fixes #967**

  If a build is executed via `cargo xtask dist --version vx.x.x`, it will now _actually_ download the schema it needs from the GitHub release artifacts page in order to build for the exact types that were built at that tagged version.

- **Allow Rover to be installed alongside Node.js v17 - @EverlastingBugstopper, #999 fixes #993**

- **Run plugin tests and lints in xtask - @EverlastingBugstopper, #977 fixes #909**

- **Adds new unused input type to check mutations - @EverlastingBugstopper, #969 fixes #967**

  Update the inputs to our check mutations to always send `null` for the `excludedOperationNames` field.

## 📚 Documentation

- **Add docs for `rover supergraph fetch` - @StephenBarlow, #971 fixes #952**

- **Miscellaneous documentation polish - @StephenBarlow, #965, #963**

# [0.4.1] - 2021-11-18

## 🚀 Features

- **Track WSL as a separate operating system from Linux - @ptondereau, #921 fixes #511**

## 🐛 Fixes

- **Properly indent multiline field comments in introspection output - @lrlna, #919 fixes #884**

- **Properly encode string values with single quotes instead of always using triple quotes - @lrlna, #930**

## 🛠 Maintenance

- **Updates `rover supergraph compose` to use `@apollo/federation@v0.33.7` - @EverlastingBugstopper, #929 fixes #924**

  This adds support for users of v16 of the `graphql` library in addition to fixing a bug in directive merging logic.

- **Expand valid node runtime range to >=14 <17 for npm installs - @EverlastingBugstopper #900 fixes #912**

## 📚 Documentation

- **Miscellaneous copy edits - @StephenBarlow, #923**

# [0.4.0] - 2021-11-02

## 🚀 Features

- **Federation 2 Support - [EverlastingBugstopper], [pull/887]**

  The alpha version of Federation 2 [has been released](https://www.apollographql.com/docs/federation/v2/)!

  In Rover, you can use the Federation 2 composition model by running `rover fed2 supergraph compose --config <supergraph.yaml>` after [installing](https://www.apollographql.com/docs/federation/v2/federation-2/moving-to-federation-2/) the `rover-fed2` binary.. You _must_ install `rover-fed2` before you can run `rover fed2 supergraph compose`, and they _must_ be the same version in order to be compatible with each other.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/887]: https://github.com/apollographql/rover/pull/887

- **Graph lifecycle management - [EverlastingBugstopper], [issue/722] [pull/861]**

  Rover now supports the `rover graph delete` command, which will delete all subgraphs in a federated variant, or delete the schema for a non-federated variant.

  There is also new documentation on how [`rover graph publish`](https://www.apollographql.com/docs/rover/graphs/#publishing-a-schema-to-apollo-studio) and [`rover subgraph publish`](https://www.apollographql.com/docs/rover/subgraphs/#publishing-a-subgraph-schema-to-apollo-studio) create new variants.

  Additionally, you no longer need to pass `--convert` to `subgraph publish` when publishing a subgraph to a new variant, though you will still need it when converting a non-federated variant to a federated variant.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/861]: https://github.com/apollographql/rover/pull/861
  [issue/722]: https://github.com/apollographql/rover/issues/722

- **Fetch the subgraph's `routing_url` from the graph registry if it isn't specified in a `supergraph.yaml` file - [EverlastingBugstopper], [pull/873]**

  Now, whenever `routing_url` is not specified for a subgraph in `supergraph.yaml` and the source is a `graphref` (which fetches a subgraph from the graph registry), the composed supergraph will use the routing URL from the graph registry for that subgraph instead of an empty string.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/873]: https://github.com/apollographql/rover/pull/873

## 🐛 Fixes

- **Output enum descriptions in introspection results - [lrlna], [issue/878] [pull/879]**

  Now, any introspection result that contains descriptions for enums will include them in the output.

  [lrlna]: https://github.com/lrlna
  [pull/879]: https://github.com/apollographql/rover/pull/879
  [issue/878]: https://github.com/apollographql/rover/issues/878

- **Output directive arguments in introspection results - [lrlna], [pull/858]**

  Now, any introspection results that contain directive arguments will include them in the output.

  [lrlna]: https://github.com/lrlna
  [pull/858]: https://github.com/apollographql/rover/pull/858

## 🛠 Maintenance

- **Refactor HTTP client configuration - [ptondereau], [issue/844] [pull/890]**

  When Rover first introduced options to configure the HTTP client, there was a function that took configuration arguments, and returned a client. This has now been refactored to use a more idiomatic [builder](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html) approach. Thanks @ptondereau!

  [ptondereau]: https://github.com/ptondereau
  [pull/890]: https://github.com/apollographql/rover/pull/890
  [issue/844]: https://github.com/apollographql/rover/issues/844

- **Updates workspace to the 2021 Rust edition - [EverlastingBugstopper], [pull/886]**

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/886]: https://github.com/apollographql/rover/pull/886

## 📚 Documentation

- **Replace 'data graph' with 'graph' in all of the docs - [trevorblades], [pull/860]**

  [trevorblades]: https://github.com/trevorblades
  [pull/860]: https://github.com/apollographql/rover/pull/860

- **Add a missing backtick to `$PATH` in the CI install docs - [EverlastingBugstopper], [pull/874]**

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/874]: https://github.com/apollographql/rover/pull/874

- **Clarify the drawbacks of publishing introspection results to the graph registry - [EverlastingBugstopper], [pull/866]**

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/866]: https://github.com/apollographql/rover/pull/866

# [0.3.0] - 2021-09-24

> Important: 1 breaking change below, indicated by **❗ BREAKING ❗**

## ❗ BREAKING ❗

- **`rover supergraph compose` uses a newer composition function that is incompatible with older versions of `@apollo/gateway` - [EverlastingBugstopper], [issue/801] [pull/832]**

  The `rover supergraph compose` command produces a supergraph schema by using composition functions from the [`@apollo/federation`](https://www.apollographql.com/docs/federation/api/apollo-federation/) package. Because that library is still in pre-1.0 releases (as are Rover and Apollo Gateway), this update to Rover means `rover supergraph compose` will create a supergraph schema with new functionality. In turn, this requires that you update your `@apollo/gateway` version to >= v0.39.x.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/832]: https://github.com/apollographql/rover/pull/832
  [issue/801]: https://github.com/apollographql/rover/issues/801

## 🚀 Features

- **Adds options to bypass TLS validation - [EverlastingBugstopper], [issue/720] [pull/837]**

  In some configurations, often on internal networks, you might need Rover to communicate over encrypted channels (e.g., HTTPS) but avoid the more stringent digital certificate verifications that validate hostnames. You might even need to bypass the digital certificate validation entirely. This is generally not recommended and considered to be much less secure but for cases where it's necessary, but now there are two flags you can use to configure how Rover validates HTTPS requests:

  - The `--insecure-accept-invalid-hostnames` flag disables hostname validation. If hostname verification is not used, any valid certificate for any site is trusted for use from any other. This introduces a significant vulnerability to person-in-the-middle attacks.

  - The `--insecure-accept-invalid-certs` flag disables certificate validation. If invalid certificates are trusted, any certificate for any site is trusted for use. This includes expired certificates. This introduces significant vulnerabilities, and should only be used as a last resort.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/837]: https://github.com/apollographql/rover/pull/837
  [issue/720]: https://github.com/apollographql/rover/issues/720

- **Adds option to increase rover's request timeout - [EverlastingBugstopper], [issue/792] [pull/838]**

  By default, Rover times out requests to the Apollo Studio API and your graph endpoints after 30 seconds. Now, if you're executing a command that might take longer than 30 seconds to process, you can increase this timeout with the `--client-timeout` option like so:

  ```sh
  rover subgraph check my-graph --validation-period 1m --client-timeout=60
  ```

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/838]: https://github.com/apollographql/rover/pull/838
  [issue/792]: https://github.com/apollographql/rover/issues/792

## 🛠 Maintenance

- **Simplify error formatting - [EverlastingBugstopper], [pull/845]**

  Now, Rover always indents the suggestion by 8 spaces instead of determining its length based on the length of the error descriptor, and the underlying cause of request errors will only be printed once.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/845]: https://github.com/apollographql/rover/pull/845

## 📚 Documentation

- **Clarify `--output json` support in migration guide, and provide an example `jq` script - [EverlastingBugstopper], [issue/839] [pull/840]**

  The Apollo CLI migration guide now mentions Rover's support for `--output json`, and our `--output json` docs now link to an example bash script for converting a check response to markdown.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/840]: https://github.com/apollographql/rover/pull/840
  [issue/839]: https://github.com/apollographql/rover/issues/839

# [0.2.1] - 2021-09-20

## 🐛 Fixes

- **Properly swallow unparseable git remotes - [EverlastingBugstopper], [issue/670] [pull/760]**

  In v0.2.0, we fixed a crash that occurred for users with non-standard git remotes. While the crash
  itself no longer occurred, the crash report itself was still generated - this is no longer the case.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/760]: https://github.com/apollographql/rover/pull/760
  [issue/670]: https://github.com/apollographql/rover/issues/670

## 🛠 Maintenance

- **Move markdown link checker to `cargo xtask lint` - [EverlastingBugstopper], [issue/774] [pull/778]**

  We now check for broken markdown links through `xtask`, meaning you can more accurately check if CI will pass locally.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/778]: https://github.com/apollographql/rover/pull/778
  [issue/774]: https://github.com/apollographql/rover/issues/774

- **Migrate lints/tests from GitHub Actions to CircleCI - [EverlastingBugstopper], [issue/774] [pull/781]**

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/781]: https://github.com/apollographql/rover/pull/781
  [issue/774]: https://github.com/apollographql/rover/issues/774

- **Run tests on centos 7 and ensure the binary only depends on glibc <= 2.18 - [EverlastingBugstopper], [pull/800]**

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/800]: https://github.com/apollographql/rover/pull/800

- **Migrate release process from GitHub Actions to CircleCI - [EverlastingBugstopper], [issue/795] [pull/808]**

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/808]: https://github.com/apollographql/rover/pull/808
  [issue/795]: https://github.com/apollographql/rover/issues/795

## 📚 Documentation

- **Clarifies setting HEAD SHA for GitHub Actions - [StephenBarlow], [pull/763]**

  Extended the [section in the docs](https://www.apollographql.com/docs/rover/ci-cd/#github-actions) for configuring GitHub Actions
  to include instructions for properly configuring the git context.

  [StephenBarlow]: https://github.com/StephenBarlow
  [pull/763]: https://github.com/apollographql/rover/pull/763

- **Fix a typo in the docs - [SaintMalik], [pull/762]**

  [SaintMalik]: https://github.com/SaintMalik
  [pull/762]: https://github.com/apollographql/rover/pull/762

# [0.2.0] - 2021-08-23

## 🚀 Features

- **Stabilize and document structured output - [EverlastingBugstopper] & [StephenBarlow], [issue/741] & [pull/750]/[pull/752]**

  Rover now has an `--output` parameter on every command that allows you to format Rover's output as well-structured JSON. Documentation for this feature can be found [here](https://www.apollographql.com/docs/rover/configuring/#--output-json).

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [StephenBarlow]: https://github.com/StephenBarlow
  [pull/750]: https://github.com/apollographql/rover/pull/750
  [pull/752]: https://github.com/apollographql/rover/pull/752
  [issue/741]: https://github.com/apollographql/rover/issues/741

- **Add an error message when an input schema is empty - [EverlastingBugstopper], [issue/724] [pull/726]**

  If the input to `--schema` was ever empty, you'd get some fairly strange and unexpected error messages. Now, if you supply an empty schema via the `--schema` argument, you'll get an error message informing you as such.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/726]: https://github.com/apollographql/rover/pull/726
  [issue/724]: https://github.com/apollographql/rover/issues/724

- **Retry HTTP requests that respond with 500-599 errors - [EverlastingBugstopper], [issue/693] [pull/727]**

  Now, by default, Rover will retry any requests that result in an internal server error for up to 10 seconds.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/727]: https://github.com/apollographql/rover/pull/727
  [issue/693]: https://github.com/apollographql/rover/issues/693

## 🐛 Fixes

- **Fix description encodings for introspection results - [lrlna], [issue/728] [pull/742]**

  Rover will now print descriptions for fields and inputs with correct spacing between triple quotes.

  [lrlna]: https://github.com/lrlna
  [pull/742]: https://github.com/apollographql/rover/pull/742
  [issue/728]: https://github.com/apollographql/rover/issues/728

- **Don't panic on git remotes without an apparent owner - [EverlastingBugstopper], [issue/670] [pull/731]**

  Most git remotes include an author and a repo name, but this isn't always the case. One of Rover's dependencies assumed this _was_ always the case, and would panic if it wasn't the case. This broke workflows for people who had these types of git remotes, but it won't anymore!

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/731]: https://github.com/apollographql/rover/pull/731
  [issue/670]: https://github.com/apollographql/rover/issues/670

- **Properly send validation period as part of checks configuration - [EverlastingBugstopper], [issue/737] [pull/738]**

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/738]: https://github.com/apollographql/rover/pull/738
  [issue/737]: https://github.com/apollographql/rover/issues/737

- **Use correct cargo target for xtask commands - [EverlastingBugstopper], [issue/582] [pull/730]**

  Any `cargo xtask` command that relies on cargo targets will now determine a correct default if building on a machine with a CPU architecture other than `x86_64`

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/730]: https://github.com/apollographql/rover/pull/730
  [issue/582]: https://github.com/apollographql/rover/issues/582

## 🛠 Maintenance

- **Add `cargo update` to `cargo xtask prep` step - [EverlastingBugstopper], [issue/746] [pull/747]**

  This change makes sure that our dependencies are automatically updated as part of our release process.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/747]: https://github.com/apollographql/rover/pull/747
  [issue/746]: https://github.com/apollographql/rover/issues/746

- **Further DRY StudioClient - [EverlastingBugstopper], [pull/753]**

  This PR removed some small inconsistencies between HTTP requests made to Apollo Studio vs. those made for user introspection requests.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/753]: https://github.com/apollographql/rover/pull/753

- **Use our GitHub bug report template for auto-generated panic reports - [EverlastingBugstopper], [issue/530] [pull/732]**

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/732]: https://github.com/apollographql/rover/pull/732
  [issue/530]: https://github.com/apollographql/rover/issues/530

## 📚 Documentation

- **Deploy Rover's docs at the root to account for main root-level redirect - [trevorblades], [pull/744]**

  This is purely a change to how Rover's docs are rolled out, no user facing changes here.

  [trevorblades]: https://github.com/trevorblades
  [pull/744]: https://github.com/apollographql/rover/pull/744


# [0.2.0-beta.1] - 2021-08-05

## 🐛 Fixes

- **Update GraphQL types to match new API Schema - [EverlastingBugstopper], [issue/696] [pull/697]**

  The Apollo Studio API introduced a change that made a field in the `subgraph publish` mutation nullable. This caused our codegen to fail and users started getting some cryptic error messages for failed publishes in older versions of Rover.

  This release handles these cases better and also introduces local tooling for building old versions of Rover with the API schemas that were in production at the time that version was published with `cargo xtask dist --release vx.x.x`.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/697]: https://github.com/apollographql/rover/pull/697
  [issue/696]: https://github.com/apollographql/rover/issues/696
  
## 📚 Documentation

- **Fix broken link to supergraph schemas - [abernix], [issue/687] [pull/706]**

  There was a broken link in our docs that now points to a set of definitions of supergraphs and subgraphs that lives in the docs for Federation.

  [abernix]: https://github.com/abernix
  [pull/706]: https://github.com/apollographql/rover/pull/706
  [issue/687]: https://github.com/apollographql/rover/issues/687

# [0.1.10] - 2021-08-05

## 🐛 Fixes

- **Update GraphQL types to match new API Schema - [EverlastingBugstopper], [issue/696] [pull/697]**

  The Apollo Studio API introduced a change that made a field in the `subgraph publish` mutation nullable. This caused our codegen to fail and users started getting some cryptic error messages for failed publishes in older versions of Rover.

  This release handles these cases better and also introduces local tooling for building old versions of Rover with the API schemas that were in production at the time that version was published with `cargo xtask dist --release vx.x.x`.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/697]: https://github.com/apollographql/rover/pull/697
  [issue/696]: https://github.com/apollographql/rover/issues/696
  
## 📚 Documentation

- **Fix broken link to supergraph schemas - [abernix], [issue/687] [pull/706]**

  There was a broken link in our docs that now points to a set of definitions of supergraphs and subgraphs that lives in the docs for Federation.

  [abernix]: https://github.com/abernix
  [pull/706]: https://github.com/apollographql/rover/pull/706
  [issue/687]: https://github.com/apollographql/rover/issues/687

# [0.2.0-beta.0] - 2021-07-26

## 🚀 Features

- **Adds structured output to Rover - [EverlastingBugstopper], [issue/285] [pull/676]**

  Rover now has an `--output` parameter on every command that allows you to format Rover's output as well-structured JSON. This structure is not set in stone and will change prior to a stable release. If you write scripts around this structured output, then you should add a check in your scripts for the top level `json_version` key, and make sure to update your scripts when that version is not what you expect (the first version is `1.beta`).

  We'd love your feedback on this new feature, or if you notice any bugs in your existing workflows, so please submit issues!

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/676]: https://github.com/apollographql/rover/pull/676
  [issue/285]: https://github.com/apollographql/rover/issues/285


# [0.1.9] - 2021-07-22

## 🚀 Features

- **Updates `@tag`/`@inaccessible` composition algorithm in `rover supergraph compose` - [trevor-scheer]/[EverlastingBugstopper], [issue/682] [pull/684]**
  - Includes simple merging/union rollup of `@tag` directives from subgraphs into a supergraph
  - Allows usage of `@tag` directives on all subgraph fields
  - Merges subgraph `@tag` directives into a supergraph
      - if _ANY_ instance is tagged, the `@tag` is union merged
        into the supergraph

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [trevor-scheer]: https://github.com/trevor-scheer
  [issue/682]: https://github.com/apollographql/rover/issues/682
  [pull/684]: https://github.com/apollographql/rover/pull/684 

- **`rover subgraph publish` and `rover subgraph delete` now acknowledges operations with no composition errors. - [EverlastingBugstopper], [issue/632] [pull/685]**

  Previously, if there were no composition errors on deletions/publishes of subgraphs, Rover would simply not display errors. Now, Rover will output a success message if there were no composition errors. This should make these types of successes more clear in CI workflows that may be publishing more than one subgraph at a time.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/685]: https://github.com/apollographql/rover/pull/685
  [issue/632]: https://github.com/apollographql/rover/issues/632

## 🛠 Maintenance

- **Make the test process fail in CI if there are test failures - [EverlastingBugstopper], [pull/683]**

  For some reason, `cargo test` was exiting with code `0` even if there were failed tests. We run tests through our own custom `xtask`, so we've added a wrapper around `cargo test` to detect and propagate the problems with our tests in our CI logs.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/683]: https://github.com/apollographql/rover/pull/683

- **Updates dependencies - [EverlastingBugstopper]/[dependabot], [pull/671], [pull/672], [pull/673], and [pull/680]**

  `anyhow` `1.0.41` -> `1.0.42`
  `cc` `1.0.68` -> `1.0.69`
  `cargo_metadata` `0.13.1` -> `0.14.0`
  `termimad` `0.13.0` -> `0.14.0`

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [dependabot]: https://github.com/dependabot
  [pull/671]: https://github.com/apollographql/rover/pull/671
  [pull/672]: https://github.com/apollographql/rover/pull/672
  [pull/673]: https://github.com/apollographql/rover/pull/673
  [pull/680]: https://github.com/apollographql/rover/pull/680

## 📚 Documentation

# [0.1.8]  2021-07-07

## 🚀 Features

- **Adds _preview_ support for `@tag` and `@inaccessible` directives - [EverlastingBugstopper], [pull/631]**

  **Preview** support for composing subgraphs with `@tag` and/or `@inaccessible` core features using `rover supergraph compose`. Note that `@apollo/gateway >= 0.33` is required when using **preview** support for these core features.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/631]: https://github.com/apollographql/rover/pull/631

- **Auto-decode gzipped responses - [EverlastingBugstopper], [pull/650]**

  If your GraphQL server responds with an introspection response compressed with brotli, it will now be decoded automatically instead of failing the command.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/650]: https://github.com/apollographql/rover/pull/650

## 🐛 Fixes

- **Use built-in root certificates and re-use HTTP connection pool - [EverlastingBugstopper], [issue/645] [pull/649]**

  Rover now uses local CA Certificates along with your operating system's native TLS implementation instead of the Rust-based WebPKI implementation.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/649]: https://github.com/apollographql/rover/pull/649
  [issue/645]: https://github.com/apollographql/rover/issues/645

## 🛠 Maintenance

- **Re-use HTTP connection pool - [EverlastingBugstopper], [pull/650]**

  Rover will now create and reuse the same HTTP connection pool for subsequent requests, which should slightly improve performance.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/650]: https://github.com/apollographql/rover/pull/650

- **Removes unused dependencies - [EverlastingBugstopper], [pull/651]**

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/651]: https://github.com/apollographql/rover/pull/651

# [0.1.7]  2021-06-29

## 🚀 Features

- **Auto-decode gzipped responses - [EverlastingBugstopper], [issue/608] [pull/620]**

  If your GraphQL server responds with a gzipped introspection response, it will now be decoded automatically instead of failing the command.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/620]: https://github.com/apollographql/rover/pull/620
  [issue/608]: https://github.com/apollographql/rover/issues/608

## 🐛 Fixes

- **Prevent update checker from aborting commands - [EverlastingBugstopper], [pull/624]**

  Previously, if there was a spurious network error when attempting to check for a newer version of Rover, the command would fail. This is no longer the case, if GitHub is down, you will still be able to run Rover commands.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/624]: https://github.com/apollographql/rover/pull/624

## 🛠 Maintenance

- **Address Clippy 0.1.53 warnings - [EverlastingBugstopper], [pull/621]**

  Updated Rover's code to conform to the latest lints.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/621]: https://github.com/apollographql/rover/pull/621

- **New `cargo xtask` command suite - [EverlastingBugstopper], [issue/388] [pull/562]**

  We've replaced a decent chunk of bash scripting in GitHub actions with Rust code. This means you can locally run most commands you need for contributing to Rover with `cargo xtask`.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/562]: https://github.com/apollographql/rover/pull/562
  [issue/388]: https://github.com/apollographql/rover/issues/388

- **Additional integration tests - [EverlastingBugstopper], [pull/629]**

  We've set up some integration tests that run `make ci` after cloning the [supergraph-demo].

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/629]: https://github.com/apollographql/rover/pull/629
  [supergraph-demo]: https://github.com/apollographql/supergraph-demo

## 📚 Documentation

- **Extend contribution guide and create an architecture document - [EverlastingBugstopper], [JakeDawkins] & [StephenBarlow], [issue/561] [pull/594]**

  Our new architecture document includes a guide on how to add a new command to Rover, and the `CONTRIBUTING.md` file at the root of the Rover repository is automatically included on our documentation site.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [StephenBarlow]: https://github.com/StephenBarlow
  [JakeDawkins]: https://github.com/JakeDawkins
  [pull/594]: https://github.com/apollographql/rover/pull/594
  [issue/561]: https://github.com/apollographql/rover/issues/561

- **Use rover@latest in BitBucket documentation - [setchy], [pull/617]**

  [setchy]: https://github.com/setchy
  [pull/617]: https://github.com/apollographql/rover/pull/617

- **Small clarifications/tweaks - [StephenBarlow], [pull/619]**

  [StephenBarlow]: https://github.com/StephenBarlow
  [pull/619]: https://github.com/apollographql/rover/pull/619

# [0.1.6]  2021-06-08

## 🐛 Fixes

- **Fix panic on empty GraphQL Error array - [EverlastingBugstopper], [issue/590] [pull/592]**

  In certain scenarios, Rover will encounter GraphQL errors, which are return as an array of strings. Previously, we were not checking if that array was empty before attempting to print the first error in the array, which caused Rover to [panic](https://doc.rust-lang.org/std/macro.panic.html). Rover has now been refactored a bit to have simpler GraphQL error handling and will no longer panic in this scenario.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/592]: https://github.com/apollographql/rover/pull/592
  [issue/590]: https://github.com/apollographql/rover/issues/590

- **Don't mangle `stderr` when an update to Rover is available - [EverlastingBugstopper], [issue/584] [pull/586]**

  Once a day, when a new version of Rover is available, it messages users letting them know that they can update. Unfortunately, this message was being printed to `stdout` instead of `stderr` due to a bug in an upstream dependency. This bug has now been fixed, and update messages are now properly routed to `stderr`.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/586]: https://github.com/apollographql/rover/pull/586
  [issue/584]: https://github.com/apollographql/rover/issues/584

## 📚 Documentation

- **Update Error Code docs title - [StephenBarlow], [pull/597]**

  "Index of Errors" -> "Rover CLI error codes"

  [StephenBarlow]: https://github.com/StephenBarlow
  [pull/597]: https://github.com/apollographql/rover/pull/597

- **Bump docs theme - [StephenBarlow], [pull/596]**

  Updates Gatsby and Apollo's Gatsby theme to match the rest of Apollo's docs.

  [StephenBarlow]: https://github.com/StephenBarlow
  [pull/596]: https://github.com/apollographql/rover/pull/596

- **Correct instance of `subgraph push` - [DNature], [pull/585]**

  Fixes an instance of `subgraph push` to be `subgraph publish`.

  [DNature]: https://github.com/DNature
  [pull/585]: https://github.com/apollographql/rover/pull/585

# [0.1.5]  2021-05-25

## 🐛 Fixes

- **Vendor OpenSSL@v1.1 - [EverlastingBugstopper], [issue/579] [pull/580]**

  Version 0.1.3 of Rover attempted to vendor OpenSSL as part of its build process, but MacOS comes preinstalled with LibreSSL instead of OpenSSL. Unfortunately, LibreSSL does not work with Rosetta 2, which allows M1 Mac users to emulate x86_64 code on their machines. Installing and specifying the correct OpenSSL version solves this problem.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/580]: https://github.com/apollographql/rover/pull/580
  [issue/579]: https://github.com/apollographql/rover/issues/579

- **Don't squash request errors - [EverlastingBugstopper], [issue/539] & [issue/573], [pull/574]**

  Rover previously had error handling for using `subgraph introspect` on an `apollo-server` instance with introspection disabled, but another attempt to handle HTTP Status Codes as errors superseded that specialized error. This case now has a much more helpful error message.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/574]: https://github.com/apollographql/rover/pull/574
  [issue/539]: https://github.com/apollographql/rover/issues/539
  [issue/573]: https://github.com/apollographql/rover/issues/573

# [0.1.4] (yanked)

# [0.1.3] - 2021-05-25

## 🐛 Fixes

- **Remove OpenSSL runtime dependency - [EverlastingBugstopper], [issue/554] & [issue/563], [pull/558]**

  Attempts to install Rover on M1 Macs were failing due to issues with OpenSSL. Issues with OpenSSL are incredibly common when writing and distributing cross-platform software, so we've attempted to remedy this issue by embedding (or vendoring) the necessary OpenSSL code directly into Rover's binaries.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/558]: https://github.com/apollographql/rover/pull/558
  [issue/554]: https://github.com/apollographql/rover/issues/554
  [issue/563]: https://github.com/apollographql/rover/issues/563

- **Remove misfired E005 error - [EverlastingBugstopper], [issue/570] [pull/571]**

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/571]: https://github.com/apollographql/rover/pull/571
  [issue/570]: https://github.com/apollographql/rover/issues/570

# [0.1.2] - 2021-05-20

## 🐛 Fixes

- **Fix unusable `rover subgraph check` - [EverlastingBugstopper], [issue/553] [pull/555]**

  Rover v0.1.1 had a regression that didn't allow `rover subgraph check` to be run on federated graphs due to a logic error. Thanks to [@setchy] for reporting this so quickly!.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [@setchy]: https://github.com/setchy
  [pull/555]: https://github.com/apollographql/rover/pull/555
  [issue/553]: https://github.com/apollographql/rover/issues/553

# [0.1.1] - 2021-05-19

## 🚀 Features

- **Prebuilt binaries for Alpine Linux - [EverlastingBugstopper], [issue/537] [pull/538]**

  Previously, Rover was only built for systems that had [`glibc`](https://www.gnu.org/software/libc/) >= 2.18 installed. This was due to the fact that we embed [v8](https://v8.dev/) into the binaries to execute the JS-powered `rover supergraph compose` command.

  Our CI pipeline now produces a statically-linked binary compiled with [`musl-libc`](https://www.musl-libc.org/) that *does not include* `rover supergraph compose`. Our installers will check if you have a compatible version of `glibc`, and if you do not, it will download the new statically linked binary and warn you that it is missing some functionality.

  We hope to bring `rover supergraph compose` to Alpine in the future, though how soon that future will come is [not yet known](https://github.com/apollographql/rover/issues/546).

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/538]: https://github.com/apollographql/rover/pull/538
  [issue/537]: https://github.com/apollographql/rover/issues/537

## 🐛 Fixes

- **No longer panic on mistyped graph names/invalid API keys - [EverlastingBugstopper], [issue/548] & [issue/550] [pull/549]**

  We received some user reports of Rover crashing if a graph name or API key was invalid. In these cases, you will now receive an actionable error message.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/549]: https://github.com/apollographql/rover/pull/549
  [issue/548]: https://github.com/apollographql/rover/issues/548
  [issue/550]: https://github.com/apollographql/rover/issues/550

## 📚 Documentation 

# [0.1.0] - 2021-05-11
> Important: 2 breaking changes below, indicated by **❗ BREAKING ❗**
## ❗ BREAKING ❗

- **Removes -V/--version flag from subcommands - [EverlastingBugstopper], [pull/487]**

  Rover's subcommands will always be the same version as Rover, so we no longer accept `-V` or `--version`
  on Rover's subcommands.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/487]: https://github.com/apollographql/rover/pull/487

- **Disallow all non-UTF-8 argument values - [EverlastingBugstopper], [pull/487]**

  Rover will no longer accept any argument values that cannot be properly interpreted as UTF-8.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/487]: https://github.com/apollographql/rover/pull/487

## 🚀 Features

- **`rover supergraph fetch` - [EverlastingBugstopper], [issue/452] [pull/485]**

  This new command allows you to fetch the latest successfully composed supergraph SDL. This can be used to bootstrap a local graph router, or to inspect the schema that is used in managed federation.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/485]: https://github.com/apollographql/rover/pull/485
  [issue/452]: https://github.com/apollographql/rover/issues/452

- **Adds link to the Apollo CLI -> Rover migration guide in `rover docs open` - [EverlastingBugstopper], [pull/492]**

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/492]: https://github.com/apollographql/rover/pull/492

- **`rover supergraph compose` allows for registry and introspection SDL sources - [lrlna], [issue/449] [pull/519]**

  Pulls subgraphs from various sources specified in the YAML config file. A valid config can now specify schema using Apollo Registry refs (`subgraph`, `graphref`), local file references (`file`) and subgraph introspection (`subgraph_url`):
  
  ```yaml
  subgraphs:
    films:
      routing_url: https://films.example.com
      schema: 
        file: ./films.graphql
    people:
      routing_url: https://example.com/people
      schema: 
        subgraph_url: https://example.com/people
    actors:
      routing_url: https://localhost:4005
      schema: 
        graphref: mygraph@current 
        subgraph: actors 
  ```
  [lrlna]: https://github.com/lrlna
  [issue/449]: https://github.com/apollographql/rover/issues/449
  [pull/519]: https://github.com/apollographql/rover/pull/519

- **`--routing-url` is now an optional argument to `rover subgraph publish` - [EverlastingBugstopper], [issue/169] [pull/484]**

  When publishing a subgraph, it is important to include a routing URL for that subgraph, so your graph router
  knows where to route requests for types in a subgraph. Previously, you had to specify this argument on
  every `rover subgraph publish`, but now it acts as an upsert, meaning you must include it on your first
  `rover subgraph publish`, but subsequent publishes will retain the existing routing URL for a subgraph
  if `--routing-url` is not specified.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/484]: https://github.com/apollographql/rover/pull/484
  [issue/169]: https://github.com/apollographql/rover/issues/169

- **`rover explain` command added - [JakeDawkins], [pull/457]**

  When encountering most errors in Rover, there will be an error code in the format
  `E###` printed along with the error description. Running `rover explain CODE`
  will now print a more detailed description of the error along with any
  resolution steps and relevant docs links.

  [JakeDawkins]: https://github.com/JakeDawkins
  [pull/457]: https://github.com/apollographql/rover/pull/457

- **Better error messages for HTTP errors - [EverlastingBugstopper], [issue/489] [pull/518]**

  Previously, Rover obfuscated the information about HTTP errors that occurred. Now, if something goes wrong between your machine and any HTTP server, you'll get some more information about what exactly went wrong.

  [Author]: https://github.com/EverlastingBugstopper
  [pull/518]: https://github.com/apollographql/rover/pull/518
  [issue/489]: https://github.com/apollographql/rover/issues/489

- **Add help text to `--log` argument - [EverlastingBugstopper], [pull/486]**

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/486]: https://github.com/apollographql/rover/pull/486

- **Updated descriptor formatting - [lrlna], [pull/533]**

  We've added some bold and extra newline spacing to the human-readable descriptors for Rover's output.

  [lrlna]: https://github.com/lrlna
  [pull/533]: https://github.com/apollographql/rover/pull/533

- **Trim down log verbosity - [EverlastingBugstopper], [pull/532]**

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/532]: https://github.com/apollographql/rover/pull/532

- **Display "unspecified" in `rover subgraph list` output instead of "N/A" - [abernix], [issue/483] [pull/505]**

  [abernix]: https://github.com/abernix
  [pull/505]: https://github.com/apollographql/rover/pull/505
  [issue/483]: https://github.com/apollographql/rover/issues/483

- **Adds `rover docs open migration` - [EverlastingBugstopper], [pull/503]**

  There is a new migration guide from the old Apollo CLI to Rover, and this command will open that page for you.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/503]: https://github.com/apollographql/rover/pull/503

## 🐛 Fixes

- **Ignore routing URL argument in telemetry - [EverlastingBugstopper], [pull/506]**

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/506]: https://github.com/apollographql/rover/pull/506

- **Print output to file without additional newline - [JakeDawkins], [issue/469] [pull/475]**

  [JakeDawkins]: https://github.com/JakeDawkins
  [pull/475]: https://github.com/apollographql/rover/pull/475
  [issue/469]: https://github.com/apollographql/rover/issues/469

## 🛠 Maintenance

- **Removes unnecessary custom URL parser - [EverlastingBugstopper], [pull/493]**

  `structopt` will automatically use the `FromStr` implementation on the `Url` type, so
  we have removed the custom parser we were previously using.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/493]: https://github.com/apollographql/rover/pull/493

- **Check for broken markdown links in CI - [EverlastingBugstopper], [issue/444] [pull/460]**

  Occasionally links get out of date (or they were mistyped in the first place) - we want to
  make sure links in this repository remain functional, so we now check for broken markdown
  links in our CI jobs that run on each push.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/460]: https://github.com/apollographql/rover/pull/460
  [issue/444]: https://github.com/apollographql/rover/issues/444

- **Addresses clippy 1.52 warnings - [EverlastingBugstopper], [pull/515]**

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/515]: https://github.com/apollographql/rover/pull/515

- **Fix credential retrieval in `rover config whoami` - [EverlastingBugstopper], [issue/514] [pull/516]**

  `rover config whoami` no longer fails if `$APOLLO_KEY` is set but there is no default authentication profile.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/516]: https://github.com/apollographql/rover/pull/516
  [issue/514]: https://github.com/apollographql/rover/issues/514

- **Point users towards issue templates instead of blank new issue page - [EverlastingBugstopper], [pull/509]**

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/509]: https://github.com/apollographql/rover/pull/509

## 📚 Documentation

- **Remove public preview section from docs - [StephenBarlow], [pull/527]**

  Rover is now generally available!

  [StephenBarlow]: https://github.com/StephenBarlow
  [pull/527]: https://github.com/apollographql/rover/pull/527

- **Document using Rover with BitBucket Pipelines - [setchy], [pull/491]**

  [setchy]: https://github.com/setchy
  [pull/491]: https://github.com/apollographql/rover/pull/491

- **Remove incorrect note about subgraph schemas - [JakeDawkins], [pull/481]**

  [JakeDawkins]: https://github.com/JakeDawkins
  [pull/481]: https://github.com/apollographql/rover/pull/481

- **Remove automated steps from release checklist - [EverlastingBugstopper], [pull/473]**

  Quite a few of the steps in our [release checklist](./RELEASE_CHECKLIST.md) have been automated as a part of our CI strategy, so those steps have been removed from the manual checklist.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/473]: https://github.com/apollographql/rover/pull/473

- **GitHub Releases page now explain how to validate the autogenerated SHA-256 checksums. - [EverlastingBugstopper], [pull/445]**

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/445]: https://github.com/apollographql/rover/pull/445

- **Update demo introspection endpoint from https to http - [abernix], [pull/534]**

  [abernix]: https://github.com/abernix
  [pull/534]: https://github.com/apollographql/rover/pull/534

- **Document Rover's inability to run on Alpine images - [lrlna], [issue/524] [pull/528]**

  [lrlna]: https://github.com/lrlna
  [pull/528]: https://github.com/apollographql/rover/pull/528
  [issue/524]: https://github.com/apollographql/rover/issues/524

- **Change "Discuss on Spectrum" link to go to Spectrum's root - [abernix], [issue/492] [pull/507]**

  [abernix]: https://github.com/abernix
  [pull/507]: https://github.com/apollographql/rover/pull/507
  [issue/492]: https://github.com/apollographql/rover/issues/492

# [0.0.10] - 2021-04-27

## 🚀 Features

- **Prints information about opting out of anonymized usage data collection after installation - [EverlastingBugstopper], [pull/456]**

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/456]: https://github.com/apollographql/rover/pull/456

- **Report SHA-256 hash of git remote URL - [EverlastingBugstopper], [issue/313] [pull/461]**

  Our anonymized usage data will now report the hash of a git remote URL if it exists to more accurately determine the number of unique projects Rover is used in.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/461]: https://github.com/apollographql/rover/pull/461
  [issue/313]: https://github.com/apollographql/rover/issues/313

- **Client returns an error on non-200 status codes - [EverlastingBugstopper], [pull/472]**

  Sometimes when performing HTTP requests, a non-200 status code is returned. This is now properly handled, and Rover's HTTP client will return a proper error message informing you of the bad response.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/472]: https://github.com/apollographql/rover/pull/472

- **Curl installer returns error message on Linux if glibc is missing - [EverlastingBugstopper], [issue/393] [pull/494]**

  Rover is currently built for the `unknown-linux-gnu` rustc target, which requires `glibc` >= 2.7 to be installed.
  Previously, if you attempted to install Rover on a machine without `glibc`, you would get quite cryptic linker
  errors. Now, users attempting to install Rover without the proper `glibc` install will get an error message
  informing them.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/494]: https://github.com/apollographql/rover/pull/494
  [issue/393]: https://github.com/apollographql/rover/issues/393

- **Better error messages when running `rover subgraph` commands on non-federated graphs - [JakeDawkins] & [lrlna], [issue/121] [pull/459]**

  You will now receive error messages for attempting to introspect a subgraph on graphs that don't support `_service`, attempting to push a subgraph to a non-federated graph, and for attempts to run `rover subgraph check` on a non-federated graph.

  [JakeDawkins]: https://github.com/JakeDawkins
  [lrlna]: https://github.com/lrlna
  [pull/459]: https://github.com/apollographql/rover/pull/459
  [issue/121]: https://github.com/apollographql/rover/issues/121

## 🐛 Fixes

- **Adds a newline to all output to stdout - [EverlastingBugstopper], [issue/458] [pull/462]**

  Rover no longer has the bug where it wouldn't conclude its output with a newline. Now we don't make your shells upset! 

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/462]: https://github.com/apollographql/rover/pull/462
  [issue/458]: https://github.com/apollographql/rover/issues/458

## 🛠 Maintenance

- **Automatically add triage labels to issues created with templates - [JakeDawkins], [pull/448]**

  [JakeDawkins]: https://github.com/JakeDawkins
  [pull/448]: https://github.com/apollographql/rover/pull/448

- **Refactor API key loading - [EverlastingBugstopper], [pull/463]**

  Made a slight performance refactor to how we load our API keys that reduces the number of filesystem reads.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/463]: https://github.com/apollographql/rover/pull/463

- **Update dependency crates - [EverlastingBugstopper], [pull/470]**

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/470]: https://github.com/apollographql/rover/pull/470

## 📚 Documentation

- **Updates language of migration guide - [StephenBarlow], [pull/446]**

  [StephenBarlow]: https://github.com/StephenBarlow
  [pull/446]: https://github.com/apollographql/rover/pull/446


# [0.0.9] - 2021-04-13

> This release is a small release to fix the automated release process, and should be considered an extension of the previous (v0.0.8) release

## 🛠 Maintenance

- ** Fix boolean logic in release action - [EverlastingBugstopper], [pull/442]**

  0.0.8 was released to npm as a beta by accident because of an environment variable being treated as a boolean when it's a string. This just fixes that for a new release.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/442]: https://github.com/apollographql/rover/pull/442

# [0.0.8] - 2021-04-13

## 🚀 Features

- **Users can now install Rover with a shorter URL - [JakeDawkins], [issue/287] [pull/426]**

  Instead of downloading Rover's install script from the tagged GitHub URL, you can now use the much simpler endpoints:

  https://rover.apollo.dev/nix/latest and https://rover.apollo.dev/win/latest.

  You can see our [documentation](https://www.apollographql.com/docs/rover/getting-started/) for more info on the new installation pattern.

  [JakeDawkins]: https://github.com/JakeDawkins
  [pull/426]: https://github.com/apollographql/rover/pull/426
  [issue/287]: https://github.com/apollographql/rover/issues/287

- **Print link to documentation after installation - [EverlastingBugstopper], [issue/141] [pull/437]**

  After a user installs Rover, we now print a link to the getting started page at
  https://go.apollo.dev/r/docs.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/437]: https://github.com/apollographql/rover/pull/437
  [issue/141]: https://github.com/apollographql/rover/issues/141

## 🐛 Fixes

- **Deserialize supergraph configuration to a deterministic data structure - [lrlna], [issue/422] [pull/423]**

  Previously, when Rover deserialized a supergraph configuration file, it did so using a HashMap.
  This made the results of `rover supergraph compose` non-deterministic, which was undesirable. 
  By switching from a HashMap to a BTreeMap, `rover supergraph compose` is now deterministic.

  [lrlna]: https://github.com/lrlna
  [pull/423]: https://github.com/apollographql/rover/pull/423
  [issue/422]: https://github.com/apollographql/rover/issues/422

## 🛠 Maintenance

- **Update telemetry URL - [JakeDawkins], [pull/427]**

  Telemetry is now routed through a Netlify function instead of a Cloudflare Worker.

  [JakeDawkins]: https://github.com/JakeDawkins
  [pull/427]: https://github.com/apollographql/rover/pull/427


## 📚 Documentation

- **Add Apollo CLI to Rover migration guide - [JakeDawkins], [issue/284] [pull/425]**

  Rover now has a migration guide for users coming from the Apollo CLI. You can see it [here](https://www.apollographql.com/docs/rover/migration).

  [jakedawkins]: https://github.com/JakeDawkins
  [pull/425]: https://github.com/apollographql/rover/pull/425
  [issue/284]: https://github.com/apollographql/rover/issues/284

# [0.0.7] - 2021-04-01
## 🐛 Fixes
- **Updates URL base in Core Schema output to specs.apollo.dev - [abernix], [pull/418]**

  [abernix]: https://github.com/abernix
  [pull/418]: https://github.com/apollographql/rover/pull/418

## 📚 Documentation
- **Added specificity to Rover's public preview period docs - [ndintenfass], [pull/415]**

  [ndintenfass]: https://github.com/ndintenfass
  [pull/415]: https://github.com/apollographql/rover/pull/415

- **Small categorization update - [StephenBarlow], [pull/414]**

  [StephenBarlow]: https://github.com/StephenBarlow
  [pull/414]: https://github.com/apollographql/rover/pull/414

# [0.0.6] - 2021-03-31
## 🚀 Features
- **Add postinstall instructions - [EverlastingBugstopper], [pull/406]**

  Adds a message after an install on how to add Rover to a user's PATH.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/406]: https://github.com/apollographql/rover/pull/406

## 🐛 Fixes
- **Change 'CSDL' to 'Core Schema' - [lrlna], [pull/403]**

  Output of `rover supergraph compose` is a Core Schema. Our output
  previously indicated that it was a `CSDL`.

  [lrlna]: https://github.com/lrlna
  [pull/403]: https://github.com/apollographql/rover/pull/403

- **Remove Rover binary before overwriting it - [EverlastingBugstopper], [issue/398] [pull/400]**

  Updating Rover with `curl` required a restart on MacOS due to Apple's
  Gatekeeper not recognizing that a new package has been installed on a
  user's system. This forces the installer to remove previously Rover
  directory and its installation and do a fresh installation.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [issue/398]: https://github.com/apollographql/rover/issues/398
  [pull/400]: https://github.com/apollographql/rover/pull/400

- **Adds entitlements to MacOS signed binaries - [EverlastingBugstopper], [issue/399] [pull/405]**

  `rover supergraph compose` process has been getting `killed` on MacOS in
  `v0.0.5` release. This was happening due to the fact that we are using
  `deno-core` to execute composition, a package that requires access to
  memory management. This fix adds an Entitelement when notarizing Rover that
  specifically allows for unsigned packages to have access to memory
  management.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [issue/399]: https://github.com/apollographql/rover/issues/399
  [pull/405]: https://github.com/apollographql/rover/pull/405


# [0.0.5] - 2021-03-30

> Important: 2 breaking changes below, indicated by **❗ BREAKING ❗**

## 🚀 Features
- **Adds introspection ability for subgraphs - [lrlna], [issue/349] [pull/377]**

  A new command, `rover subgraph introspect` has been added. This command
  runs a _federated introspection_ query against a server which has
  implemented the requirements of the [federation
  specification](https://www.apollographql.com/docs/federation/federation-spec/).
  This command accepts endpoint headers (`-H`, `--header`) for making the introspection
  request (if required) and outputs SDL to stdout.

  [lrlna]: https://github.com/lrlna
  [pull/377]: https://github.com/apollographql/rover/pull/377
  [issue/349]: https://github.com/apollographql/rover/issues/349

- **Fallback to monochromic output in installer when `tput` is unavailable - [abernix], [issue/371] [pull/372]**

  The `tput` command allows easier ANSI output using named values in rather than
  control characters.

  While we could just use control characters and maintain colored output in the
  absence of `tput`, it's probably also reasonable to gracefully fall back to
  monochromatic output.

  [abernix]: https://github.com/abernix
  [pull/372]: https://github.com/apollographql/rover/pull/372
  [issue/371]: https://github.com/apollographql/rover/issues/371

## ❗ BREAKING ❗
- **Renames `core build` to `supergraph compose` - [lrlna], [pull/391]**

  To align with other Apollo teams on the usage of `supergraph` and
  `composition`, we are renaming `core build` to `supergraph compose`.

  [lrlna]: https://github.com/lrlna
  [pull/391]: https://github.com/apollographql/rover/pull/391

- **Updates harmonizer@0.2.2 - [abernix], [pull/396]**

  Updates harmonizer to the latest version. This version now composes and
  returns a core schema instead of CSDL. CSDL was an internal implementation
  of composition and this new format is meant to bring some stability to `rover
  supergraph compose`.

  [abernix]: https://github.com/abernix
  [pull/396]: https://github.com/apollographql/rover/pull/396

## 🐛 Fixes
- **Handle 400-599 HTTP responses - [lrlna], [issue/394] [issue/187] [pull/395]**

  Previously, Rover did not provide errors for any HTTP requests that return a status code between 400 and 599. This fix makes sure Rover checks for those errors before moving on to parsing the response body.

  This fix also does an extra check for 400 errors, as the Apollo
  Server sends additional information that we can display to users.

  [lrlna]: https://github.com/lrlna
  [issue/394]: https://github.com/apollographql/rover/issues/394
  [issue/187]: https://github.com/apollographql/rover/issues/187
  [pull/395]: https://github.com/apollographql/rover/pull/395

## 🛠 Maintenance
- **Sign and notarize MacOS binaries as part of CI - [EverlastingBugstopper], [pull/363]**

  This automates our signing and notarization process when releasing MacOS
  binaries. This is especially necessary to install and run Rover on latest
  M1s and Big Sur.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/363]: https://github.com/apollographql/rover/pull/363

- **Test, build and release on ubuntu-16.04 - [abernix], [pull/381]**

  This pins us to Ubuntu 16.04 which ships with glib 2.19. This should allow
  us to work with a wider range of operating systems than the newer glib
  that we get with Ubuntu 20.04, which is ubuntu-latest on GitHub Actions
  Virtual Environments (which resulted in a Rover that wouldn't run on
  Ubuntu 18.04).

  Ubuntu 16.04 is LTS until April 2024, and is still receiving active
  updates through the LTS program.

  [abernix]: https://github.com/abernix
  [pull/381]: https://github.com/apollographql/rover/pull/381

- **Cache Rust artifacts in CI linter job- [EverlastingBugstopper], [pull/365]**

  The rest of our GitHub actions workflows pull from the cache to take
  advantage of Rust's incremental compilation. We now do this for clippy
  too so it finishes (and fails) faster.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/365]: https://github.com/apollographql/rover/pull/365

- **Addresses new clippy 1.51 warning - [EverlastingBugstopper], [pull/364]**

  Addresses some stylistic problems noticed by the new version of our linter [clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-151)

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/364]: https://github.com/apollographql/rover/pull/364

## 📚 Documentation
- **Update documentation for 0.0.5 release- [JakeDawkins] [StephenBarlow] [EverlastingBugstopper], [pull/389]**

  Documents recent additions to Rover in detail, including `rover supergraph
  compose`, `rover subgraph introspect` and `rover graph introspect`.

  [JakeDawkins]: https://github.com/JakeDawkins
  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [StephenBarlow]: https://github.com/StephenBarlow
  [pull/389]: https://github.com/apollographql/rover/pull/389

# [0.0.4] - 2021-03-23

> Important: Two breaking changes below, indicated by **❗ BREAKING ❗**

## 🚀 Features

- **Core schema building capabilities - [EverlastingBugstopper], [pull/340]**

  Adds a new command, `rover core build` to handle building 
  [core schema documents](https://specs.apollo.dev/#core-schemas)
  from multiple subgraph schemas. This also adds a new config format to support
  this command in YAML. Currently, this is only documented in [pull/340].

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/340]: https://github.com/apollographql/rover/pull/340

- **Adds introspection ability for graphs - [lrlna], [issue/180] [issue/171] [pull/283]**

  A new command, `rover graph introspect` has been added, usable for introspecting
  graphs (not subgraphs). This command accepts endpoint [headers] for making the
  introspection request (if required) and outputs SDL to stdout.

  [lrlna]: https://github.com/lrlna
  [pull/283]: https://github.com/apollographql/rover/pull/283
  [issue/180]: https://github.com/apollographql/rover/issues/180
  [issue/171]: https://github.com/apollographql/rover/issues/171
  [headers]: https://github.com/apollographql/rover/pull/351

## ❗ BREAKING ❗

- **Rename `push` to `publish` everywhere - [JakeDawkins], [issue/344] [pull/347]**

  "Publish" is a more correct name for what these commands do. We wanted to be
  extra clear about its name matching its functionality, so any usage of `push`
  commands should now be `publish`.

  [JakeDawkins]: https://github.com/JakeDawkins
  [pull/347]: https://github.com/apollographql/rover/pull/347
  [issue/344]: https://github.com/apollographql/rover/issues/344

- **Rename `committer` to `author` - [EverlastingBugstopper], [issue/338] [pull/339]**

  Note: This is only  breaking change if you were previously using the `APOLLO_VCS_COMMITTER` 
  env variable. If so, migrate by changing that environment variable to `APOLLO_VCS_AUTHOR`.

  Changes the underlying git context implementation to report the `author` of a commit rather than a `committer`. This is primarily intended to properly link the real author with a commit in Apollo Studio.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/339]: https://github.com/apollographql/rover/pull/339
  [issue/338]: https://github.com/apollographql/rover/issues/338

## 🐛 Fixes

- **Output composition errors to stderr - [EverlastingBugstopper], [pull/335]**

  There was an incorrect usage of `tracing::error`, causing composition errors
  to not show up when expected. This change unifies this error printing with the
  rest of the project.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/335]: https://github.com/apollographql/rover/pull/335

## 🛠 Maintenance

- **Auto-bump versions in install scripts - [EverlastingBugstopper], [pull/332]**

  Added auto version bumping in the build script, so there's no chance the Rover
  team will miss this important step in the release process.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/332]: https://github.com/apollographql/rover/pull/332

- **Don't print by default for automatic update checks - [EverlastingBugstopper], [pull/342]**

  When Rover automatically checks for updates every 24 hours, it no longer will
  print if there is no update available. It will still print for manual checks
  and if Rover is out of date.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/342]: https://github.com/apollographql/rover/pull/342

- **Add metadata to `Cargo.toml` - [EverlastingBugstopper], [pull/346]**

  In preparation for future releases to [crates.io](https://crates.io), we've
  added relevant metadata to the Cargo.toml file for the project.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/346]: https://github.com/apollographql/rover/pull/346

## 📚 Documentation

- **Adds `APOLLO_HOME` variable to docs - [EverlastingBugstopper], [pull/341]**

  `APOLLO_HOME` is the override variable for Rover's parent directory, where the
  binary lives. This was previously undocumented, but now it's not!

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/341]: https://github.com/apollographql/rover/pull/341



# [0.0.3] - 2021-03-09

## 🚀 Features

- **❗ BREAKING ❗ Squash `config show` functionality into `config whoami` - [EverlastingBugstopper], [issue/274] [pull/323]**

  Since the only thing that `rover config show` did was show the saved api key,
  it made sense to squash that functionality into the `whoami` command. We decided
  that we'd prefer not to ever expose the full api key to stdout (you can still
  find it in the saved config file), but we still show the first and last 4 
  characters of it to help with debugging.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/323]: https://github.com/apollographql/rover/pull/323
  [issue/274]: https://github.com/apollographql/rover/issues/274

- **Add api key origin to `whoami` command - [EverlastingBugstopper], [issue/273] [pull/307]**

  The `whoami` command, which is used to verify api keys and help with debugging now
  shows where that key came from, either a `--profile` or the `APOLLO_KEY` env variable.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/307]: https://github.com/apollographql/rover/pull/307
  [issue/273]: https://github.com/apollographql/rover/issues/273

- **`rover docs` commands to make viewing documentation easier - [EverlastingBugstopper], [issue/308] [pull/314]**

  To make it easier to find and navigate Rover's docs, we added two commands: 
  `rover docs list` to list helpful docs pages and `rover docs open` to open a
  docs page in the browser.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/314]: https://github.com/apollographql/rover/pull/314
  [issue/308]: https://github.com/apollographql/rover/issues/308

- **Better errors and suggestions for invalid variants - [EverlastingBugstopper], [issue/208] [pull/316]**

  Previously, Rover would tell you if you tried accessing an invalid variant, 
  but couldn't provide any recommendations. This adds recommendations for simple
  typos, lists available variants for graphs with small numbers of variants, and
  provides a link to view variants in Apollo Studio for graphs with many variants.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/316]: https://github.com/apollographql/rover/pull/316
  [issue/208]: https://github.com/apollographql/rover/issues/208

- **Remove the need to reload terminal after install - [EverlastingBugstopper], [issue/212] [pull/318]**
  
  Rather than asking users to reload their terminal after install, we do the
  extra work of sourcing Rover's env file after install, preventing linux users
  from having to do that or reload the terminal themselves.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/318]: https://github.com/apollographql/rover/pull/318
  [issue/212]: https://github.com/apollographql/rover/issues/212

- **Rover automatically checks for updates - [JakeDawkins], [issue/296] [pull/319]**

  Every 24 hours, Rover will automatically check for new releases and let you know.
  You can also run the `rover update check` command to manually check for updates.
  If an update is available, Rover warns once per day at most and provides a link
  to the docs for update instructions.

  [JakeDawkins]: https://github.com/JakeDawkins
  [pull/319]: https://github.com/apollographql/rover/pull/319
  [issue/296]: https://github.com/apollographql/rover/issues/296

- **Update installers to be consistent and not require version variables - [JakeDawkins], [issue/88] [pull/324]**

  This provides a consistent experience when installing Rover. When running the
  linux install script, you no longer are required to pass a `VERSION`, but still
  may if you want to download an older version. The windows installer now supports
  the same `$Env:VERSION` environment variable for similar overrides. By default,
  installer scripts will download the version of rover released with that version
  of the script.

  [JakeDawkins]: https://github.com/JakeDawkins
  [pull/324]: https://github.com/apollographql/rover/pull/324
  [issue/88]: https://github.com/apollographql/rover/issues/88

- **Verify paths are all valid utf-8 - [EverlastingBugstopper], [pull/326]**

  Just to make our code more safe and easier to maintain, we now check and make
  sure paths are all valid utf-8 to make sure any non utf-8 paths won't cause unexpected issues.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/326]: https://github.com/apollographql/rover/pull/326

## 🐛 Fixes

- **Consistently refer to configuration instead of config in error messages - [EverlastingBugstopper], [pull/306]**

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/306]: https://github.com/apollographql/rover/pull/306

## 🛠 Maintenance

- **Move all build-time checks for env variables to util - [EverlastingBugstopper], [pull/310]**

  Having a bunch of `env!` macros across the codebase is just less beautiful and
  maintainable than having them in one utility file. This PR just moves all of those
  calls, looking up `CARGO_ENV_*` environment variables to a single place.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/310]: https://github.com/apollographql/rover/pull/310

- **Make output tables prettier - [EverlastingBugstopper], [pull/315]**

  Replaces the characters in table borders with characters that show fewer &
  smaller gaps to make tables look a little more polished :)

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/315]: https://github.com/apollographql/rover/pull/315

- **Add test to make sure install scripts never change names/paths - [EverlastingBugstopper], [pull/321]**

  This adds a simple test to make sure we don't move or rename install scripts
  on accident in the future, since that would be a major breaking change.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/321]: https://github.com/apollographql/rover/pull/321

## 📚 Documentation

- **Instructions for using Rover in CircleCI and GitHub Actions - [JakeDawkins], [issue/245] [pull/329]**

  Some CI providers require a couple of additional steps to get Rover installed
  and working. These docs help get Rover working with linux setups in GitHub
  Actions and CircleCI.

  [JakeDawkins]: https://github.com/JakeDawkins
  [pull/329]: https://github.com/apollographql/rover/pull/329
  [issue/245]: https://github.com/apollographql/rover/issues/245


# [0.0.2] - 2021-02-23

## 🚀 Features

- **Better logging experience - [EverlastingBugstopper], [pull/263]**

  When passing `--log debug`, the logs are now pretty printed with their call location.

  Additionally, progress messages are no longer printed with an `INFO` prefix on every line,
  messages are displayed to the user with no mess and no fuss.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/263]: https://github.com/apollographql/rover/pull/263

- **Add useful info to debug logs - [EverlastingBugstopper], [pull/268]**

  When running Rover with `--log debug`, you can now see which environment variables are being used
  and the raw JSON payload returned by the Apollo Studio API.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/268]: https://github.com/apollographql/rover/pull/268

- **Provide a better error message for malformed API Keys - [EverlastingBugstopper], [issue/215] [pull/275]**

  Before, if you passed a malformed API key, the error message was "406: Not Acceptable", since that's
  what the Apollo Studio API returned. Rover now provides you with a
  much more actionable error message.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/275]: https://github.com/apollographql/rover/pull/275
  [issue/215]: https://github.com/apollographql/rover/issues/215

- **Add support for M1 Macbooks - [EverlastingBugstopper], [issue/295] [pull/297]/[pull/300]**

  Big Sur allows the new M1 Macbooks to run code compiled for the `x86_64` architecture in emulation
  mode, even though the machines themselves have an `arm64` architecture. We have updated
  our `curl | sh` installer and our `npm` installer to reflect this, and anybody running Big Sur
  on the new M1 machines can now install and use Rover.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/297]: https://github.com/apollographql/rover/pull/297
  [pull/300]: https://github.com/apollographql/rover/pull/300
  [issue/295]: https://github.com/apollographql/rover/issues/295

- **Add a `> ` prompt to the `rover config auth` command - [EverlastingBugstopper], [issue/279] [pull/281]**

  It was a bit confusing to be presented with a blank line after running `rover config auth`.
  To make it more clear that this is a prompt for an API key, we now print `> ` at the beginning
  of the prompt.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/281]: https://github.com/apollographql/rover/pull/281
  [issue/279]: https://github.com/apollographql/rover/issues/279

- **Add a friendlier message for the case of no config profiles - [JakeDawkins], [issue/202] [pull/303]**

  The new user experience, where there are no config profiles found, was a little cryptic.
  To make it easier to understand what the problem is, we added a friendly error message
  asking the user to run `rover config auth`.

  [JakeDawkins]: https://github.com/JakeDawkins
  [pull/303]: https://github.com/apollographql/rover/pull/303
  [issue/202]: https://github.com/apollographql/rover/issues/202

- **Output Service title for graph keys in whoami command - [lrlna], [issue/280] [pull/299]**

  `rover config whoami` was displaying `Name` information which was unclear
  in the context of this command. Instead of `Name`, we are now displaying
  `Service title` information for graph keys, and omitting `Name` and
  `Service Title` for user keys, as the already existing information provides
  enough information for `User`.

  [lrlna]: https://github.com/lrlna
  [pull/299]: https://github.com/apollographql/rover/pull/299
  [issue/280]: https://github.com/apollographql/rover/issues/280

## 🐛 Fixes

- **Allow Rover to be used outside the context of a git repository - [JakeDawkins], [issue/271] [pull/282]**

  [JakeDawkins]: https://github.com/JakeDawkins
  [pull/282]: https://github.com/apollographql/rover/pull/282
  [issue/271]: https://github.com/apollographql/rover/issues/271

- **Always use the shorthand ref when generating Git Context - [lrlna], [pull/255]**

  Rover now computes the shorthand ref and specifies that as the "branch", even
  if the specific ref is not necessarily a branch (such as a tag).

  [lrlna]: https://github.com/lrlna
  [pull/255]: https://github.com/apollographql/rover/pull/255

- **Do not send telemetry events for dev builds - [EverlastingBugstopper], [pull/258]**

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/258]: https://github.com/apollographql/rover/pull/258

- **Fix a typo in the README - [abernix], [pull/269]**

  `s/Rover co[ma]{3}nds to interact/Rover commands to interact/`

  [abernix]: https://github.com/abernix
  [pull/269]: https://github.com/apollographql/rover/pull/269

## 🛠 Maintenance

- **Address latest style suggestions (clippy) - [lrlna], [pull/267]**

  A new version of [clippy](https://rust-lang.github.io/rust-clippy/rust-1.50.0/index.html)
  gave us some pointers for more idiomatic code style, and we addressed them!

  [lrlna]: https://github.com/lrlna
  [pull/267]: https://github.com/apollographql/rover/pull/267

- **Unify terminal text color dependency - [EverlastingBugstopper], [pull/276]**

  Now we only use `ansi_term` for providing colored text.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/276]: https://github.com/apollographql/rover/pull/276

- **Hide API key printing in debug logs - [JakeDawkins], [pull/302]**

  We no longer print a user's api key in the `--log debug` logs when
  saving a key (from `rover config auth`)/

  [JakeDawkins]: https://github.com/JakeDawkins
  [pull/302]: https://github.com/apollographql/rover/pull/302

## 📚 Documentation

- **Document Git Context - [JakeDawkins], [pull/262]**

  We added documentation for how Rover provides Git Context to Apollo Studio.
  You can read all about it [here](https://apollographql.com/docs/rover/configuring/#git-context).

  [JakeDawkins]: https://github.com/JakeDawkins
  [pull/262]: https://github.com/apollographql/rover/pull/262

- **Fix npx usage documentation - [abernix], [pull/270]**

  We updated the docs to show that it is necessary to pass 
  `--package @apollo/rover` each time Rover is invoked through `npx`.

  [abernix]: https://github.com/abernix
  [pull/270]: https://github.com/apollographql/rover/pull/270

- **Update layout of Rover's intro article - [StephenBarlow], [pull/259]**

  The intro article in Rover's docs were reordered to put the info about the Public preview
  towards the bottom of the page, so more relevant information is no longer below the fold.

  [StephenBarlow]: https://github.com/StephenBarlow
  [pull/259]: https://github.com/apollographql/rover/pull/259
# [0.0.1] - 2021-02-09

**Initial beta release.** Please visit [our documentation page](https://apollographql.com/docs/rover/) for information on usage.
