load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    sha256 = "5c2b6745236f8ce547f82eeacbbcc81d736734cc8bd92e60d3e3cdfa6e167bb5",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.15.0/rules_rust-v0.15.0.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rust_repositories")

rust_repositories(edition="2018")

load("@rules_rust//proto:repositories.bzl", "rust_proto_repositories")

rust_proto_repositories()

load("@rules_rust//proto:transitive_repositories.bzl", "rust_proto_transitive_repositories")

rust_proto_transitive_repositories()

load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")

git_repository(
    name = "google_remote_apis",
    remote = "https://github.com/bazelbuild/remote-apis.git",
    commit = "aa29b91f336b9be2c5370297210b67a6654c0b72",
)

# Needed for the googleapis protos.
# Version of googleapis is taken from remote-apis repository
# to ensure compatibility.
http_archive(
    name = "googleapis",
    #There's no need to explicitly set path to external/BUILD.googleapis
    #as bazel automatically assumes all BUILD files for external dependencies
    #described in WORKSPACE file are placed in external/ directory in
    #the root folder of the project
    build_file = "BUILD.googleapis",
    sha256 = "7b6ea252f0b8fb5cd722f45feb83e115b689909bbb6a393a873b6cbad4ceae1d",
    strip_prefix = "googleapis-143084a2624b6591ee1f9d23e7f5241856642f4d",
    urls = ["https://github.com/googleapis/googleapis/archive/143084a2624b6591ee1f9d23e7f5241856642f4d.zip"],
)

http_archive(
    name = "com_github_grpc_grpc",
    sha256 = "b391a327429279f6f29b9ae7e5317cd80d5e9d49cc100e6d682221af73d984a6",
    strip_prefix = "grpc-93e8830070e9afcbaa992c75817009ee3f4b63a0",  # v1.24.3 with fixes
    urls = ["https://github.com/grpc/grpc/archive/93e8830070e9afcbaa992c75817009ee3f4b63a0.zip"],
)
