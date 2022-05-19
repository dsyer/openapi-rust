use std::collections::BTreeMap;

use std::str;

use serde_json;

use k8s_openapi::api::apps::v1::*;
use k8s_openapi::api::core::v1::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::*;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn xform(json: &str) -> String {
    let mut deployment: Deployment = serde_json::from_str(json).unwrap();
    deployment.metadata = deployment_metadata(
        Some(deployment.metadata),
        String::from("demo"),
    ); // Randomize?
    let app = if let Some(value) = deployment
        .metadata
        .labels
        .as_ref()
        .unwrap()
        .get(&String::from("app"))
    {
        value
    } else {
        "demo"
    };
    deployment.spec = Some(spec(deployment.spec, app));
    return serde_json::to_string(&deployment).unwrap();
}

fn spec(
    spec: Option<DeploymentSpec>,
    app: &str,
) -> DeploymentSpec {
    let mut result = if let Some(value) = spec {
        value
    } else {
        DeploymentSpec {
            min_ready_seconds: None,
            paused: None,
            progress_deadline_seconds: None,
            replicas: None,
            revision_history_limit: None,
            selector: LabelSelector {
                match_expressions: None,
                match_labels: None,
            }
            .into(),
            strategy: None,
            template: PodTemplateSpec {
                metadata: None,
                spec: None,
            }
            .into(),
        }
        .into()
    };
    result.selector = selector(result.selector, app);
    result.template = template(result.template, app);
    return result;
}

fn template(
    template: PodTemplateSpec,
    app: &str,
) -> PodTemplateSpec {
    let mut result = template.clone();
    result.metadata = Some(deployment_metadata(result.metadata, app.to_string()));
    result.spec = Some(pod_spec(result.spec));
    return result;
}

fn pod_spec(spec: Option<PodSpec>) -> PodSpec {
    return match spec {
        Some(value) => value,
        None => {
            let mut containers: Vec<Container> = Vec::new();
            containers.push(Container {
                args: None,
                command: None,
                env: None,
                env_from: None,
                image: Some(String::from("nginx")),
                image_pull_policy: None,
                lifecycle: None,
                liveness_probe: None,
                name: String::from("nginx"),
                ports: None,
                readiness_probe: None,
                resources: None,
                security_context: None,
                startup_probe: None,
                stdin: None,
                stdin_once: None,
                termination_message_path: None,
                termination_message_policy: None,
                tty: None,
                volume_devices: None,
                volume_mounts: None,
                working_dir: None,
            });
            PodSpec {
                active_deadline_seconds: None,
                affinity: None,
                automount_service_account_token: None,
                containers: containers,
                dns_config: None,
                dns_policy: None,
                enable_service_links: None,
                ephemeral_containers: None,
                host_aliases: None,
                host_ipc: None,
                host_network: None,
                host_pid: None,
                hostname: None,
                image_pull_secrets: None,
                init_containers: None,
                node_name: None,
                node_selector: None,
                overhead: None,
                preemption_policy: None,
                priority: None,
                priority_class_name: None,
                readiness_gates: None,
                restart_policy: None,
                runtime_class_name: None,
                scheduler_name: None,
                security_context: None,
                service_account: None,
                service_account_name: None,
                set_hostname_as_fqdn: None,
                share_process_namespace: None,
                subdomain: None,
                termination_grace_period_seconds: None,
                tolerations: None,
                topology_spread_constraints: None,
                volumes: None,
            }
            .into()
        }
    };
}

fn selector(
    selector: LabelSelector,
    app: &str,
) -> LabelSelector {
    let mut result = selector;
    match result.match_expressions {
        Some(_) => {}
        None => {
            result.match_labels = match_labels(result.match_labels, app);
        }
    }
    return result.into();
}

fn match_labels(
    labels: Option<BTreeMap<String, String>>,
    app: &str,
) -> Option<BTreeMap<String, String>> {
    let mut result: BTreeMap<String, String> = if let Some(value) = labels {
        value.clone()
    } else {
        BTreeMap::new()
    };
    if !result.contains_key(&String::from("app")) {
        result.insert(String::from("app"), app.to_string());
    }
    return Some(result);
}

fn deployment_metadata(
    metadata: Option<ObjectMeta>,
    app: String,
) -> ObjectMeta {
    let mut result = if let Some(value) = metadata {
        value
    } else {
        ObjectMeta {
            annotations: None,
            cluster_name: None,
            creation_timestamp: None,
            deletion_grace_period_seconds: None,
            deletion_timestamp: None,
            finalizers: None,
            generate_name: None,
            generation: None,
            labels: None,
            managed_fields: None,
            name: None,
            namespace: None,
            owner_references: None,
            resource_version: None,
            self_link: None,
            uid: None,
        }
        .into()
    };
    result.labels = if let Some(value) = result.labels {
        Some(value)
    } else {
        Some(BTreeMap::new())
    };
    result.labels = Some(labels(result.labels.unwrap(), app));
    return result.into();
}

fn labels(labels: BTreeMap<String, String>, app: String) -> BTreeMap<String, String> {
    let mut result = labels.clone();
    if !result.contains_key(&String::from("app")) {
        result.insert(String::from("app"), app);
    }
    return result;
}
