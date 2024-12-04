use std::fmt::{Display, Formatter};
use serde_with::NoneAsEmptyString;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct VMParameter {
    pub processors: VMProcessors,
    pub memory: VMMemory,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct VMInformation {
    pub id: String,
    pub cpu: Option<VMCPU>,
    pub memory: Option<VMMemory>,
}

#[serde_as]
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

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct VMApplianceView {
    #[serde_as(as = "NoneAsEmptyString")]
    pub author: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    pub version: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    pub port: Option<Port>,
    #[serde(rename = "showAtPowerOn")]
    #[serde_as(as = "NoneAsEmptyString")]
    pub show_at_power_on: Option<String>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct VMConnectedDevice {
    pub index: Option<Number>,
    #[serde(rename = "startConnected")]
    pub start_connected: Option<bool>,
    #[serde(rename = "connectionStatus")]
    pub connection_status: Option<Number>,
    #[serde(rename = "devicePath")]
    #[serde_as(as = "NoneAsEmptyString")]
    pub device_path: Option<String>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct VMConnectedDeviceList {
    pub num: Option<Number>,
    pub devices: Option<Vec<VMConnectedDevice>>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct VMGuestIsolation {
    #[serde(rename = "copyDisabled")]
    pub copy_disabled: Option<bool>,
    #[serde(rename = "dndDisabled")]
    pub dnd_disabled: Option<bool>,
    #[serde(rename = "hgfsDisabled")]
    pub hgfs_disabled: Option<bool>,
    #[serde(rename = "pasteDisabled")]
    pub paste_disabled: Option<bool>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct VMUsbDevice {
    pub index: Option<Number>,
    pub connected: Option<bool>,
    #[serde(rename = "backingInfo")]
    #[serde_as(as = "NoneAsEmptyString")]
    pub backing_info: Option<String>,
    #[serde(rename = "BackingType")]
    pub backing_type: Option<Number>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct VMUsbList {
    pub num: Option<Number>,
    #[serde(rename = "usbDevices")]
    pub usb_devices: Option<Vec<VMUsbDevice>>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct VMRemoteVNC {
    #[serde(rename = "VNCEnabled")]
    pub vnc_enabled: Option<bool>,
    #[serde(rename = "VNCPort")]
    pub vnc_port: Option<Port>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct VMRegisterParameter {
    pub name: String,
    pub path: String,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct VMRegistrationInformation {
    pub id: String,
    pub path: String,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigVMParamsParameter {
    #[serde_as(as = "NoneAsEmptyString")]
    pub name: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    pub value: Option<String>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct VMCloneParameter {
    pub name: String,
    #[serde(rename = "parentId")]
    pub parent_id: String,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct VMID {
    pub id: String,
    pub path: String,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub enum VMPowerState {
    #[serde(rename = "poweredOn")]
    On,
    #[serde(rename = "poweredOff")]
    Off,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "suspended")]
    Suspend,
    #[serde(rename = "poweringOn")]
    PoweringOn
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct VMPowerStateWrapper {
    pub power_state: VMPowerState,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct VMCPU {
    pub processors: VMProcessors,
}

pub type VMProcessors = i32;
pub type VMMemory = i32;
#[serde_as]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
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

impl<'a> Into<&'a str> for VMPowerOperation {
    fn into(self) -> &'a str {
        match self {
            VMPowerOperation::On => "on",
            VMPowerOperation::Off => "off",
            VMPowerOperation::Shutdown => "shutdown",
            VMPowerOperation::Suspend => "suspend",
            VMPowerOperation::Pause => "pause",
            VMPowerOperation::Unpause => "unpause",
        }
    }
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct SharedFolder {
    pub folder_id: String,
    pub host_path: String,
    pub flags: i32,
}

pub type SharedFolders = Vec<SharedFolder>;
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct SharedFolderParameter {
    pub host_path: String,
    pub flags: i32,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct NICDevice {
    pub index: NICIndex,
    #[serde(rename = "type")]
    pub nic_type: NICType,
    pub vmnet: String,
    #[serde(rename = "macAddress")]
    pub mac_address: String,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct NICDevices {
    pub num: NICNumber,
    pub nics: Vec<NICDevice>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct NICDeviceParameter {
    #[serde(rename = "type")]
    pub nic_type: NICType,
    #[serde_as(as = "NoneAsEmptyString")]
    pub vmnet: Option<String>,
}

#[serde_as]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum NICType {
    #[serde(rename = "nat")]
    Nat,
    #[serde(rename = "hostOnly")]
    HostOnly,
    #[serde(rename = "bridged")]
    Bridged,
    #[serde(rename = "custom")]
    Segment,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateVmnetParameter {
    pub name: String,
    #[serde(rename = "type")]
    pub network_type: NetworkType,
}

#[serde_as]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum NetworkType {
    #[serde(rename = "nat")]
    Nat,
    #[serde(rename = "hostOnly")]
    HostOnly,
    /// Cannot be used to create a virtual network!
    #[serde(rename = "bridged")]
    Bridged,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Network {
    pub name: String,
    #[serde(rename = "type")]
    pub network_type: NetworkType,
    pub dhcp: String,
    pub subnet: String,
    pub mask: String,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Networks {
    pub num: Number,
    pub vmnets: Vec<Network>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct MACToIP {
    pub vmnet: String,
    pub mac: String,
    pub ip: String,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct MACToIPs {
    pub num: Number,
    pub mactoips: Vec<MACToIP>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct GuestInfo {
    pub ip: String,
    pub port: Port,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Portforward {
    pub port: Port,
    pub protocol: String,
    pub desc: String,
    pub guest: GuestInfo,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Portforwards {
    pub num: Number,
    pub port_forwardings: Vec<Portforward>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct PortforwardParameter {
    #[serde(rename = "guestIp")]
    pub guest_ip: String,
    // #[serde(rename = "guestPort")]
    // pub guest_port: Port,
    #[serde_as(as = "NoneAsEmptyString")]
    pub desc: Option<String>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct NicIpStack {
    pub mac: MacAddress,
    pub ip: Option<Vec<IPNetAddress>>,
    pub dns: Option<DnsConfig>,
    pub wins: Option<WinsConfig>,
    pub dhcp4: Option<DhcpConfig>,
    pub dhcp6: Option<DhcpConfig>,
}
#[serde_as]
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
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct DnsConfig {
    #[serde_as(as = "NoneAsEmptyString")]
    pub hostname: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    pub domainname: Option<String>,
    pub server: Option<Vec<String>>,
    pub search: Option<Vec<String>>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct DhcpConfig {
    pub enabled: bool,
    pub setting: String,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct WinsConfig {
    pub primary: String,
    pub secondary: String,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct RouteEntry {
    pub dest: IPAddress,
    pub prefix: Number,
    #[serde_as(as = "NoneAsEmptyString")]
    pub nexthop: Option<IPAddress>,
    pub interface: Number,
    #[serde(rename = "type")]
    pub route_type: Number,
    pub metric: Number,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub enum DaemonState {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct MacToIPParameter {
    #[serde(rename = "IP")]
    pub ip: String,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct VMRestAPIError {
    #[serde(rename = "Code")]
    pub code: i32,
    #[serde(rename = "Message")]
    pub message: String,
}

impl Display for VMRestAPIError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error[{}]: {}", self.code, self.message)
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct VMRestAPIResponse {
    #[serde(rename = "Code")]
    pub code: i32,
    #[serde(rename = "Message")]
    pub message: String,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct VMIPAddress {
    pub ip: String
}
