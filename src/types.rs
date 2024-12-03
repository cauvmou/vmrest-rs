use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VMParameter {
    pub processors: VMProcessors,
    pub memory: VMMemory,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VMInformation {
    pub id: String,
    pub cpu: Option<VMCPU>,
    pub memory: Option<VMMemory>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VMRestrictionsInformation {
    pub id: String,
    #[serde(rename = "managedOrg")]
    pub managed_org: Option<String>,
    pub integrityconstraint: Option<String>,
    pub cpu: Option<VMCPU>,
    pub memory: Option<VMMemory>,
    #[serde(rename = "applianceView")]
    pub appliance_view: Option<VMApplianceView>,
    #[serde(rename = "cddvdList")]
    pub cddvd_list: Option<VMConnectedDeviceList>,
    #[serde(rename = "floopyList")]
    pub floopy_list: Option<VMConnectedDeviceList>,
    #[serde(rename = "firewareType")]
    pub fireware_type: Option<Number>,
    #[serde(rename = "guestIsolation")]
    pub guest_isolation: Option<VMGuestIsolation>,
    pub niclist: Option<NICDevices>,
    #[serde(rename = "parallelPortList")]
    pub parallel_port_list: Option<VMConnectedDeviceList>,
    #[serde(rename = "serialPortList")]
    pub serial_port_list: Option<VMConnectedDeviceList>,
    #[serde(rename = "usbList")]
    pub usb_list: Option<VMUsbList>,
    #[serde(rename = "remoteVNC")]
    pub remote_vnc: Option<VMRemoteVNC>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VMApplianceView {
    pub author: Option<String>,
    pub version: Option<String>,
    pub port: Option<Port>,
    #[serde(rename = "showAtPowerOn")]
    pub show_at_power_on: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VMConnectedDevice {
    pub index: Option<Number>,
    #[serde(rename = "startConnected")]
    pub start_connected: Option<String>,
    #[serde(rename = "connectionStatus")]
    pub connection_status: Option<Number>,
    #[serde(rename = "devicePath")]
    pub device_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VMConnectedDeviceList {
    pub num: Option<Number>,
    pub devices: Option<Vec<VMConnectedDevice>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VMGuestIsolation {
    #[serde(rename = "copyDisabled")]
    pub copy_disabled: Option<String>,
    #[serde(rename = "dndDisabled")]
    pub dnd_disabled: Option<String>,
    #[serde(rename = "hgfsDisabled")]
    pub hgfs_disabled: Option<String>,
    #[serde(rename = "pasteDisabled")]
    pub paste_disabled: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VMUsbDevice {
    pub index: Option<Number>,
    pub connected: Option<String>,
    #[serde(rename = "backingInfo")]
    pub backing_info: Option<String>,
    #[serde(rename = "BackingType")]
    pub backing_type: Option<Number>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VMUsbList {
    pub num: Option<Number>,
    #[serde(rename = "usbDevices")]
    pub usb_devices: Option<Vec<VMUsbDevice>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VMRemoteVNC {
    #[serde(rename = "VNCEnabled")]
    pub vnc_enabled: Option<String>,
    #[serde(rename = "VNCPort")]
    pub vnc_port: Option<Port>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VMRegisterParameter {
    pub name: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VMRrgistrationInformation {
    pub id: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigVMParamsParameter {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VMCloneParameter {
    pub name: String,
    #[serde(rename = "parentId")]
    pub parent_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VMID {
    pub id: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VMPowerState {
    pub power_state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VMCPU {
    pub processors: Option<VMProcessors>,
}

pub type VMProcessors = i32;
pub type VMMemory = i32;

#[derive(Debug, Serialize, Deserialize)]
pub enum VMPowerOperation {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "shutdown")]
    Shutdown,
    #[serde(rename = "suspend")]
    Suspend,
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "unpause")]
    Unpause,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SharedFolder {
    pub folder_id: String,
    pub host_path: String,
    pub flags: i32,
}

pub type SharedFolders = Vec<SharedFolder>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SharedFolderParameter {
    pub host_path: String,
    pub flags: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NICDevice {
    pub index: NICIndex,
    #[serde(rename = "type")]
    pub nic_type: String,
    pub vmnet: String,
    #[serde(rename = "macAddress")]
    pub mac_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NICDevices {
    pub num: NICNumber,
    pub nics: Vec<NICDevice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NICDeviceParameter {
    #[serde(rename = "type")]
    pub nic_type: String,
    pub vmnet: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateVmnetParameter {
    pub name: String,
    #[serde(rename = "type")]
    pub network_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Network {
    pub name: String,
    #[serde(rename = "type")]
    pub network_type: String,
    pub dhcp: String,
    pub subnet: String,
    pub mask: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Networks {
    pub num: Number,
    pub vmnets: Vec<Network>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MACToIP {
    pub vmnet: String,
    pub mac: String,
    pub ip: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MACToIPs {
    pub num: Number,
    pub mactoips: Vec<MACToIP>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuestInfo {
    pub ip: String,
    pub port: Port,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Portforward {
    pub port: Port,
    pub protocol: String,
    pub desc: String,
    pub guest: GuestInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Portforwards {
    pub num: Number,
    pub port_forwardings: Vec<Portforward>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortforwardParameter {
    #[serde(rename = "guestIp")]
    pub guest_ip: String,
    #[serde(rename = "guestPort")]
    pub guest_port: Port,
    pub desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NicIpStack {
    pub mac: MacAddress,
    pub ip: Option<Vec<IPNetAddress>>,
    pub dns: Option<DnsConfig>,
    pub wins: Option<WinsConfig>,
    pub dhcp4: Option<DhcpConfig>,
    pub dhcp6: Option<DhcpConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NicIpStackAll {
    pub nics: Option<NicIpStack>,
    pub routes: Option<Vec<RouteEntry>>,
    pub dns: Option<DnsConfig>,
    pub wins: Option<WinsConfig>,
    pub dhcpv4: Option<DhcpConfig>,
    pub dhcpv6: Option<DhcpConfig>,
}

pub type Number = i32;
pub type NICNumber = i32;
pub type NICIndex = i32;
pub type Port = i16;
pub type MacAddress = String;
pub type IPAddress = String;
pub type IPNetAddress = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsConfig {
    pub hostname: Option<String>,
    pub domainname: Option<String>,
    pub server: Option<Vec<String>>,
    pub search: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DhcpConfig {
    pub enabled: bool,
    pub setting: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WinsConfig {
    pub primary: String,
    pub secondary: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteEntry {
    pub dest: IPAddress,
    pub prefix: Number,
    pub nexthop: Option<IPAddress>,
    pub interface: Number,
    #[serde(rename = "type")]
    pub route_type: Number,
    pub metric: Number,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DaemonState {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MacToIPParameter {
    #[serde(rename = "IP")]
    pub ip: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorModel {
    pub code: i32,
    pub message: String,
}