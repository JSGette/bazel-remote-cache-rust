package(default_visibility = ["//visibility:public"])

load("@rules_rust//rust:defs.bzl", "rust_binary")
load("@rules_rust//proto:proto.bzl", "rust_proto_library", "rust_grpc_library")

rust_binary(
    name = "main",
    srcs = ["main.rs"]
)

rust_grpc_library(
    name = "re_grpc",
    deps = ["@google_remote_apis//build/bazel/remote/execution/v2:remote_execution_proto"]
)
