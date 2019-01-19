/// The address of the system message bus is given in the DBUS_SYSTEM_BUS_ADDRESS environment variable.
/// If that variable is not set, applications should try to connect to the well-known address unix:path=/var/run/dbus/system_bus_socket
const WELL_KNOWN_DBUS_SYSTEM_BUS_ENV: &str = "DBUS_SYSTEM_BUS_ADDRESS";

/// The address of the system message bus is given in the DBUS_SYSTEM_BUS_ADDRESS environment variable.
/// If that variable is not set, applications should try to connect to the well-known address unix:path=/var/run/dbus/system_bus_socket
const WELL_KNOWN_DBUS_SYSTEM_BUS_ADDRESS: &str = "unix:path=/var/run/dbus/system_bus_socket";
