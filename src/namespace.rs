#[allow(non_camel_case_types)]
pub enum Namespace {
    ipc(String),
    mnt(String),
    net(String),
    pid(String),
}

impl Namespace {
    pub fn from_string(input: &str) -> Option<Self> {
        let input: Vec<&str> = input.split(":").collect();
        if input[1].starts_with("[") {
            let ns_value: Vec<&str> = input[1].split("]").collect();
            match input[0] {
                "ipc" => return Some(Namespace::ipc(ns_value[0].to_string().replace("[", ""))),
                "mnt" => return Some(Namespace::mnt(ns_value[0].to_string().replace("[", ""))),
                "net" => return Some(Namespace::net(ns_value[0].to_string().replace("[", ""))),
                "pid" => return Some(Namespace::pid(ns_value[0].to_string().replace("[", ""))),
                _ => return None,
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct ContainerNamespace {
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
