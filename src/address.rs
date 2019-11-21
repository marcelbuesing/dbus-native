/// The address of the system message bus is given in the DBUS_SYSTEM_BUS_ADDRESS environment variable.
/// If that variable is not set, applications should try to connect to the well-known address unix:path=/var/run/dbus/system_bus_socket
const WELL_KNOWN_DBUS_SYSTEM_BUS_ENV: &str = "DBUS_SYSTEM_BUS_ADDRESS";

/// The address of the system message bus is given in the DBUS_SYSTEM_BUS_ADDRESS environment variable.
/// If that variable is not set, applications should try to connect to the well-known address unix:path=/var/run/dbus/system_bus_socket
const WELL_KNOWN_DBUS_SYSTEM_BUS_ADDRESS: &str = "unix:path=/var/run/dbus/system_bus_socket";

trait ServerAddress {
    fn to_address(&self) -> String;
}

struct UnixDomainSocketAddr {
    ///  Directory in which a socket file with a random file
    /// name starting with 'dbus-' will be created by the server.
    /// This key can only be used in server addresses, not in client
    /// addresses; the resulting client address will have the "path" key
    /// instead. be set.
    pub path: Option<String>,
    /// The same as "dir", except that on platforms with abstract
    /// sockets, the server may attempt to create an abstract
    /// socket whose name starts with this directory instead of a
    /// path-based socket. This key can only be used in server
    /// addresses, not in client addresses; the resulting client address
    /// will have the "abstract" or "path" key instead.
    pub tmpdir: Option<String>,
    /// Unique string in the abstract namespace, often syntactically resembling
    /// a path but unconnected to the filesystem namespace.
    /// This key is only supported on platforms with abstract Unix sockets,
    /// of which Linux is the only known example.
    pub r#abstract: Option<String>,
    /// If given, This key can only be used in server addresses,
    /// not in client addresses. If set, its value must be yes.
    /// This is typically used in an address string like
    /// unix:runtime=yes;unix:tmpdir=/tmp so that there can be a
    /// fallback if XDG_RUNTIME_DIR is not set.
    pub runtime: Option<String>,
}

impl ServerAddress for UnixDomainSocketAddr {
    fn to_address(&self) -> String {
        let mut pairs = Vec::new();

        if let Some(path) = self.path.as_ref() {
            pairs.push(format!("path={}", path));
        }

        if let Some(tmpdir) = self.tmpdir.as_ref() {
            pairs.push(format!("tmpdir={}", tmpdir));
        }

        if let Some(r#abstract) = self.r#abstract.as_ref() {
            pairs.push(format!("abstract={}", r#abstract));
        }

        if let Some(runtime) = self.runtime.as_ref() {
            pairs.push(format!("runtime={}", runtime));
        }
        format!("unix:{}", pairs.join(";"))
    }
}

struct TcpSocketAddr {
    /// DNS name or IP address
    pub host: Option<String>,
    /// Used in a listenable address to configure the interface on which the server will listen:
    /// either the IP address of one of the local machine's interfaces (most commonly 127.0.0.1 ),
    /// or a DNS name that resolves to one of those IP addresses, or '*' to listen on all
    /// interfaces simultaneously. If not specified, the default is the same value as "host".
    pub bind: Option<String>,
    /// The tcp port the server will open. A zero value let the server choose a free port
    /// provided from the underlaying operating system. libdbus is able to retrieve the real used port from the server.
    pub port: Option<u16>,
    /// If set, provide the type of socket family either "ipv4" or "ipv6".
    /// If unset, the family is unspecified.
    pub family: Option<String>,
}

impl ServerAddress for TcpSocketAddr {
    fn to_address(&self) -> String {
        let mut pairs = Vec::new();

        if let Some(host) = self.host.as_ref() {
            pairs.push(format!("host={}", host));
        }

        if let Some(bind) = self.bind.as_ref() {
            pairs.push(format!("bind={}", bind));
        }

        if let Some(port) = self.port.as_ref() {
            pairs.push(format!("port={}", port));
        }

        if let Some(family) = self.family.as_ref() {
            pairs.push(format!("family={}", family));
        }
        format!("tcp:{}", pairs.join(";"))
    }
}
