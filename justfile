NAME := "crabwalk"
VERSION := `git rev-parse HEAD`
SEMVER_VERSION := `grep version Cargo.toml | awk -F"\"" '{print $2}' | head -n 1`
NAMESPACE := "default"
KUBE_VERSION := "1.25"
SLACK_TOKEN := env_var_or_default("SLACK_TOKEN", "")
GITHUB_TOKEN := env_var_or_default("GITHUB_TOKEN", "")

default:
  @just --list --unsorted --color=always | rg -v "    default"

# delete kind
delete-kind:
	kind delete cluster && sleep 5

# start kind
start-kind:
	kind create cluster --config testdata/kind-{{KUBE_VERSION}}.yaml
	sleep 10
	kubectl wait pods --for=condition=Ready --timeout=300s --all --all-namespaces

# run
run:
  RUST_LOG=debug,kube=debug,controller=debug cargo run

# run cargo watch
watch:
	SLACK_TOKEN={{SLACK_TOKEN}} GITHUB_TOKEN={{GITHUB_TOKEN}} RUST_LOG=debug cargo watch -x 'run'

# format with nightly rustfmt
fmt:
  cargo +nightly fmt
