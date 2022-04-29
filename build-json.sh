#!/bin/bash

mkdir -p target

kubectl get --raw /openapi/v2 | \
  jq 'with_entries(select([.key] | inside(["definitions", "components", "info", "swagger", "openapi"]))) + {paths:{}}' \
  > openapi/k8s.json
