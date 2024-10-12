#[allow(non_camel_case_types)]
enum Namespace {
    ipc(String),
    mnt(String),
    net(String),
    pid(String),
}

impl Namespace {
    fn from_string(input: &str) -> Option<Self> {
        if input.starts_with("cgroup:[") && input.ends_with("]") {
            let parts: Vec<&str> = input[7..input.len()-1].split(':').collect();
            if parts.len() == 2 {
                match parts[0] {
                    "ipc" => return Some(Namespace::ipc(parts[1].to_string())),
                    "mnt" => return Some(Namespace::mnt(parts[1].to_string())),
                    "net" => return Some(Namespace::net(parts[1].to_string())),
                    "pid" => return Some(Namespace::pid(parts[1].to_string())),
                    _ => return None,
                }
            }
        }
        None
    }
}

struct ContainerNamespace {
    pub container_id: String,
    pub ipc_namespace: String,
    pub mnt_namespace: String,
    pub net_namespace: String,
    pub pid_namespace: String,

}

impl ContainerNamespace{
    pub fn new(container_id: String, namespaces : Vec<Namespace>) -> Self {
        let mut ipc_namespace = String::new();
        let mut mnt_namespace = String::new();
        let mut net_namespace = String::new();
        let mut pid_namespace = String::new();

        for ns in namespaces {
            match ns {
                Namespace::ipc(value) => ipc_namespace = value,
                Namespace::mnt(value) => mnt_namespace = value,
                Namespace::net(value) => net_namespace = value,
                Namespace::pid(value) => pid_namespace = value,
            }
        }

        ContainerNamespace {
            container_id,
            ipc_namespace,
            mnt_namespace,
            net_namespace,
            pid_namespace,
        }
    }
}
