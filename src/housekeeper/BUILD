package(default_visibility = ["//visibility:public"])

load("@rules_rust//rust:defs.bzl", "rust_library")

rust_library(
    name = "housekeeper_lib",
    srcs = [
        "housekeeper.rs",
    ],
    deps = [
        "//src:re_grpc"
    ]
)
