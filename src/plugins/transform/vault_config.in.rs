// This is not a standalone Rust module.  It gets processed by serde to
// generate serialization code and included directly into another module.

/// How should our applications authenticate themselves with vault?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum AuthType {
    /// Issue time-limited VAULT_TOKEN values to each container, setting
    /// appropriate policies on each token.
    #[serde(rename = "token")]
    Token,
}

/// The policies associated with a specific pod.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ContainerConfig {
    /// Policies to apply to this container.
    policies: Vec<dc::RawOr<String>>,
}

/// Policies to apply to each container in a pod.
type PodConfig = BTreeMap<String, ContainerConfig>;

/// The configuration for our Vault plugin.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    /// The kind of authentication to use.
    auth_type: AuthType,

    /// Extra environment variables to inject into each container.
    extra_environment: BTreeMap<String, dc::RawOr<String>>,

    /// Default policies to apply to every container.
    default_policies: Vec<dc::RawOr<String>>,

    /// More specific policies to apply to individual
    pods: BTreeMap<String, PodConfig>,
}

#[test]
fn can_deserialize_config() {
    let f = fs::File::open("examples/rails_hello/config/vault.yml").unwrap();
    let config: Config = serde_yaml::from_reader(f).unwrap();
    assert_eq!(config.auth_type, AuthType::Token);
}
