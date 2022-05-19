```
$ mkdir tmp; git clone https://github.com/kubernetes-client/gen tmp/gen
$ pip install urllib3
```

Generate raw JSON:

```
$ rm -rf openapi-k8s; mkdir openapi-k8s
$ kubectl get --raw /openapi/v2 | jq 'with_entries(select([.key] | inside(["definitions", "components", "info", "swagger", "openapi"]))) + {paths:{}}' > openapi-k8s/swagger.json.unprocessed
```

Preprocess:

```
$ OPENAPI_MODEL_LENGTH=2 KUBERNETES_CRD_GROUP_PREFIX=com.example OPENAPI_SKIP_FETCH_SPEC=true KUBERNETES_CRD_MODE=true python ./tmp/gen/openapi/preprocess_spec.py java v1.19.1 openapi-k8s/swagger.json kubernetes kubernetes
```

Generate:

```
$ cp tmp/gen/openapi/java.xml openapi-k8s/pom.xml; cd openapi-k8s/
$ LIBRARY=native OPENAPI_SKIP_BASE_INTERFACE=true KUBERNETES_CRD_MODE=true mvn -D openapi-generator-version=6.0.0-beta -D generator.package.name=com.dsyer -D generator.client.version=0.0.1 clean generate-sources
```

Attempt to run CLI outside Maven (would probably work for some languages):

```
$ jbang org.openapitools:openapi-generator-cli:6.0.0-beta generate -g java -o openapi-k8s -i openapi-k8s/swagger.json
```