#![allow(unused_qualifications)]

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;


/// Slave address (primary or secondary)
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Address(String);

impl std::convert::From<String> for Address {
    fn from(x: String) -> Self {
        Address(x)
    }
}

impl std::string::ToString for Address {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for Address {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(Address(x.to_string()))
    }
}

impl std::convert::From<Address> for String {
    fn from(x: Address) -> Self {
        x.0
    }
}

impl std::ops::Deref for Address {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for Address {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


impl Address {
    /// Helper function to allow us to convert this model to an XML string.
    /// Will panic if serialisation fails.
    #[allow(dead_code)]
    pub(crate) fn to_xml(&self) -> String {
        serde_xml_rs::to_string(&self).expect("impossible to fail to serialize")
    }
}

/// Baudrate to use for the communication - valid values 300, 600, 1200, 2400, 4800, 9600
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum Baudrate { 
    #[serde(rename = "300")]
    _300,
    #[serde(rename = "600")]
    _600,
    #[serde(rename = "1200")]
    _1200,
    #[serde(rename = "2400")]
    _2400,
    #[serde(rename = "4800")]
    _4800,
    #[serde(rename = "9600")]
    _9600,
}

impl std::fmt::Display for Baudrate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self { 
            Baudrate::_300 => write!(f, "{}", "300"),
            Baudrate::_600 => write!(f, "{}", "600"),
            Baudrate::_1200 => write!(f, "{}", "1200"),
            Baudrate::_2400 => write!(f, "{}", "2400"),
            Baudrate::_4800 => write!(f, "{}", "4800"),
            Baudrate::_9600 => write!(f, "{}", "9600"),
        }
    }
}

impl std::str::FromStr for Baudrate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "300" => std::result::Result::Ok(Baudrate::_300),
            "600" => std::result::Result::Ok(Baudrate::_600),
            "1200" => std::result::Result::Ok(Baudrate::_1200),
            "2400" => std::result::Result::Ok(Baudrate::_2400),
            "4800" => std::result::Result::Ok(Baudrate::_4800),
            "9600" => std::result::Result::Ok(Baudrate::_9600),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

impl Baudrate {
    /// Helper function to allow us to convert this model to an XML string.
    /// Will panic if serialisation fails.
    #[allow(dead_code)]
    pub(crate) fn to_xml(&self) -> String {
        serde_xml_rs::to_string(&self).expect("impossible to fail to serialize")
    }
}

/// The device the M-Bus is connected to - /dev/ is prepended to {device} by M-Bus HTTPD
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Device(String);

impl std::convert::From<String> for Device {
    fn from(x: String) -> Self {
        Device(x)
    }
}

impl std::string::ToString for Device {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for Device {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(Device(x.to_string()))
    }
}

impl std::convert::From<Device> for String {
    fn from(x: Device) -> Self {
        x.0
    }
}

impl std::ops::Deref for Device {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for Device {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


impl Device {
    /// Helper function to allow us to convert this model to an XML string.
    /// Will panic if serialisation fails.
    #[allow(dead_code)]
    pub(crate) fn to_xml(&self) -> String {
        serde_xml_rs::to_string(&self).expect("impossible to fail to serialize")
    }
}

/// Raspberry Pi Hat Information
// Methods for converting between header::IntoHeaderValue<Hat> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Hat>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Hat>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Hat - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Hat> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Hat as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Hat - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Hat {
    /// Product
    #[serde(rename = "product")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub product: Option<String>,

    /// Product ID
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub product_id: Option<String>,

    /// Product Version
    #[serde(rename = "productVer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub product_ver: Option<String>,

    /// Hat UUID
    #[serde(rename = "uuid")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub uuid: Option<String>,

    /// Hat Vendor
    #[serde(rename = "vendor")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vendor: Option<String>,

}

impl Hat {
    pub fn new() -> Hat {
        Hat {
            product: None,
            product_id: None,
            product_ver: None,
            uuid: None,
            vendor: None,
        }
    }
}

/// Converts the Hat value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Hat {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref product) = self.product {
            params.push("product".to_string());
            params.push(product.to_string());
        }


        if let Some(ref product_id) = self.product_id {
            params.push("productId".to_string());
            params.push(product_id.to_string());
        }


        if let Some(ref product_ver) = self.product_ver {
            params.push("productVer".to_string());
            params.push(product_ver.to_string());
        }


        if let Some(ref uuid) = self.uuid {
            params.push("uuid".to_string());
            params.push(uuid.to_string());
        }


        if let Some(ref vendor) = self.vendor {
            params.push("vendor".to_string());
            params.push(vendor.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Hat value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Hat {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub product: Vec<String>,
            pub product_id: Vec<String>,
            pub product_ver: Vec<String>,
            pub uuid: Vec<String>,
            pub vendor: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Hat".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "product" => intermediate_rep.product.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "productId" => intermediate_rep.product_id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "productVer" => intermediate_rep.product_ver.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "uuid" => intermediate_rep.uuid.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "vendor" => intermediate_rep.vendor.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Hat".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Hat {
            product: intermediate_rep.product.into_iter().next(),
            product_id: intermediate_rep.product_id.into_iter().next(),
            product_ver: intermediate_rep.product_ver.into_iter().next(),
            uuid: intermediate_rep.uuid.into_iter().next(),
            vendor: intermediate_rep.vendor.into_iter().next(),
        })
    }
}


impl Hat {
    /// Helper function to allow us to convert this model to an XML string.
    /// Will panic if serialisation fails.
    #[allow(dead_code)]
    pub(crate) fn to_xml(&self) -> String {
        serde_xml_rs::to_string(&self).expect("impossible to fail to serialize")
    }
}

/// Max frames to listen for
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Maxframes(i32);

impl std::convert::From<i32> for Maxframes {
    fn from(x: i32) -> Self {
        Maxframes(x)
    }
}


impl std::convert::From<Maxframes> for i32 {
    fn from(x: Maxframes) -> Self {
        x.0
    }
}

impl std::ops::Deref for Maxframes {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl std::ops::DerefMut for Maxframes {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}


impl Maxframes {
    /// Helper function to allow us to convert this model to an XML string.
    /// Will panic if serialisation fails.
    #[allow(dead_code)]
    pub(crate) fn to_xml(&self) -> String {
        serde_xml_rs::to_string(&self).expect("impossible to fail to serialize")
    }
}

/// M-Bus device data as an XML document
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MbusData(String);

impl std::convert::From<String> for MbusData {
    fn from(x: String) -> Self {
        MbusData(x)
    }
}

impl std::string::ToString for MbusData {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for MbusData {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(MbusData(x.to_string()))
    }
}

impl std::convert::From<MbusData> for String {
    fn from(x: MbusData) -> Self {
        x.0
    }
}

impl std::ops::Deref for MbusData {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for MbusData {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


impl MbusData {
    /// Helper function to allow us to convert this model to an XML string.
    /// Will panic if serialisation fails.
    #[allow(dead_code)]
    pub(crate) fn to_xml(&self) -> String {
        serde_xml_rs::to_string(&self).expect("impossible to fail to serialize")
    }
}

/// Output of libmbus scan command
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Slaves(String);

impl std::convert::From<String> for Slaves {
    fn from(x: String) -> Self {
        Slaves(x)
    }
}

impl std::string::ToString for Slaves {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for Slaves {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(Slaves(x.to_string()))
    }
}

impl std::convert::From<Slaves> for String {
    fn from(x: Slaves) -> Self {
        x.0
    }
}

impl std::ops::Deref for Slaves {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for Slaves {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


impl Slaves {
    /// Helper function to allow us to convert this model to an XML string.
    /// Will panic if serialisation fails.
    #[allow(dead_code)]
    pub(crate) fn to_xml(&self) -> String {
        serde_xml_rs::to_string(&self).expect("impossible to fail to serialize")
    }
}

/// Some error text
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TextError(String);

impl std::convert::From<String> for TextError {
    fn from(x: String) -> Self {
        TextError(x)
    }
}

impl std::string::ToString for TextError {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for TextError {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(TextError(x.to_string()))
    }
}

impl std::convert::From<TextError> for String {
    fn from(x: TextError) -> Self {
        x.0
    }
}

impl std::ops::Deref for TextError {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for TextError {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


impl TextError {
    /// Helper function to allow us to convert this model to an XML string.
    /// Will panic if serialisation fails.
    #[allow(dead_code)]
    pub(crate) fn to_xml(&self) -> String {
        serde_xml_rs::to_string(&self).expect("impossible to fail to serialize")
    }
}

/// A YAML file
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Yaml(String);

impl std::convert::From<String> for Yaml {
    fn from(x: String) -> Self {
        Yaml(x)
    }
}

impl std::string::ToString for Yaml {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for Yaml {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(Yaml(x.to_string()))
    }
}

impl std::convert::From<Yaml> for String {
    fn from(x: Yaml) -> Self {
        x.0
    }
}

impl std::ops::Deref for Yaml {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for Yaml {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


impl Yaml {
    /// Helper function to allow us to convert this model to an XML string.
    /// Will panic if serialisation fails.
    #[allow(dead_code)]
    pub(crate) fn to_xml(&self) -> String {
        serde_xml_rs::to_string(&self).expect("impossible to fail to serialize")
    }
}
