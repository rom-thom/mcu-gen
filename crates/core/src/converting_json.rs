
use serde::{Serialize, Deserialize};
use svd_parser::svd::Device; // To be Serializable means to be able to convert to for example json (For some reason Device isn't Serializable)



#[derive(Deserialize, Clone, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SerializableAccess { // I tok this from the Access enum in Device
    /// Read access is permitted. Write operations have an undefined effect.
    ReadOnly,
    /// Read and write accesses are permitted.
    ReadWrite,
    /// Read access is always permitted.
    /// Only the first write after a reset will affect the content.
    /// Following writes have an undefined effect.
    ReadWriteOnce,
    /// Read operations have undefined results.
    /// Only the first write after a reset will affect the content.
    WriteOnce,
    /// Read operations have an undefined result. Write access is permitted.
    WriteOnly,
}

#[derive(Deserialize, Clone, Copy, Serialize)]
#[serde(rename_all = "kebab-case")]
enum ReadAction {
    Clear,
    Set,
    Modify,
}

#[derive(Deserialize, Clone, Copy, Serialize)]
#[serde(rename_all = "kebab-case")]
enum ModifiedWriteValues {
    OneToClear,
    OneToSet,
    OneToToggle,
    ZeroToClear,
    ZeroToSet,
    ZeroToToggle,
    Modify,
}


#[derive(Deserialize, Clone, Copy, Serialize)]
#[serde(rename_all = "kebab-case")]
enum SerializableUsage {
    Read,
    Write,
    ReadWrite,
}


#[derive(Deserialize, Clone, Debug, Serialize)]
struct SerializableEnumeratedValues {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage: Option<SerializableUsage>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    values: Vec<SerializableEnumValue>,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
struct SerializableEnumValue {
    name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    value: u64,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
struct SerializableWriteConstraint {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    minimum: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    maximum: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    enumerated: Option<Vec<u64>>,
}


#[derive(Deserialize, Clone, Debug, Serialize)]
struct SerializableField {
    name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    bit_offset: u32,
    bit_width: u32,

    // Extended metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    access: Option<SerializableAccess>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    derived_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    modified_write_values: Option<ModifiedWriteValues>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    write_constraint: Option<SerializableWriteConstraint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_action: Option<ReadAction>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    enumerated_values: Vec<SerializableEnumeratedValues>,
}



#[derive(Deserialize, Clone, Debug, Serialize)]
struct SerializableRegister {
    name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    address_offset: u32,

    // Technical metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    size: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    access: Option<SerializableAccess>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reset_value: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reset_mask: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    derived_from: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    alternate_group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    alternate_register: Option<String>,

    // Bitfield structure
    #[serde(skip_serializing_if = "Vec::is_empty")]
    fields: Vec<SerializableField>,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
struct SerializablePeripheral {
    name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    group_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    alternate_peripheral: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    derived_from: Option<String>,

    base_address: u64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    registers: Vec<SerializableRegister>,

    #[serde(skip_serializing_if = "Option::is_none")]
    prepend_to_name: Option<String>, // to be able to change the name for example with name collition
}



#[derive(Deserialize, Clone, Debug, Serialize)]
struct SerializableInterrupt {
    name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    value: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    peripheral: Option<String> // To trace where it came from
}

#[derive(Deserialize, Clone, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SerializableEndian {
    Little,
    Big,
    /// Mixed endian.
    Selectable,
    Other,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
struct SerializableCpu {
    name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    revision: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    endian: Option<SerializableEndian>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    mpu_present: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    fpu_present: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    nvic_priority_bits: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    vendor_systick_config: Option<bool>,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
struct SerializableDevice {
    name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    vendor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    vendor_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    series: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    
    cpu: Option<SerializableCpu>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    peripherals: Vec<SerializablePeripheral>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    interrupts: Vec<SerializableInterrupt>,

    // Metadata for output generation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    header_definitions_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    license_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    tool_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    generated_date: Option<String>,
}





// To convert from Device to Serializable Device

impl From<Device> for SerializableDevice{
    fn from(value: Device) -> Self {
        std::todo!("Convert from Device to make it Serializable")
    }
}