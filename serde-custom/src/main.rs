use serde::de::{self, MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Debug, PartialEq)]
struct KubeConfig {
    port: u16,
    healthz_port: u16,
    max_pods: u16,
}

// 🔹 SERIALIZATION IMPLEMENTATION
impl Serialize for KubeConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("KubeConfig", 3)?;
        state.serialize_field("port", &self.port)?;
        state.serialize_field("healthz_port", &self.healthz_port)?;
        state.serialize_field("max_pods", &self.max_pods)?;
        state.end()
    }
}

// 🔹 DESERIALIZATION IMPLEMENTATION
impl<'de> Deserialize<'de> for KubeConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["port", "healthz_port", "max_pods"];

        struct KubeConfigVisitor;

        impl<'de> Visitor<'de> for KubeConfigVisitor {
            type Value = KubeConfig;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct KubeConfig")
            }

            fn visit_map<V>(self, mut map: V) -> Result<KubeConfig, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut port = None;
                let mut healthz_port = None;
                let mut max_pods = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "port" => {
                            if port.is_some() {
                                return Err(de::Error::duplicate_field("port"));
                            }
                            port = Some(map.next_value()?);
                        }
                        "healthz_port" => {
                            if healthz_port.is_some() {
                                return Err(de::Error::duplicate_field("healthz_port"));
                            }
                            healthz_port = Some(map.next_value()?);
                        }
                        "max_pods" => {
                            if max_pods.is_some() {
                                return Err(de::Error::duplicate_field("max_pods"));
                            }
                            max_pods = Some(map.next_value()?);
                        }
                        _ => {
                            let _: de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                let port = port.ok_or_else(|| de::Error::missing_field("port"))?;
                let healthz_port =
                    healthz_port.ok_or_else(|| de::Error::missing_field("healthz_port"))?;
                let max_pods = max_pods.ok_or_else(|| de::Error::missing_field("max_pods"))?;

                Ok(KubeConfig {
                    port,
                    healthz_port,
                    max_pods,
                })
            }
        }

        deserializer.deserialize_struct("KubeConfig", FIELDS, KubeConfigVisitor)
    }
}

fn main() {
    println!("Hello, world!");

    let kubconfig = KubeConfig {
        port: 8081,
        healthz_port: 9091,
        max_pods: 10,
    };

    // 🔹 Test SERIALIZATION
    let config_str = serde_json::to_string(&kubconfig).unwrap();
    println!("Serialized: {}", config_str);
    // Output: Serialized: {"port":8081,"healthz_port":9091,"max_pods":10}

    // 🔹 Test DESERIALIZATION
    let deserialized: KubeConfig = serde_json::from_str(&config_str).unwrap();
    println!("Deserialized: {:?}", deserialized);
    // Output: Deserialized: KubeConfig { port: 8081, healthz_port: 9091, max_pods: 10

    // 🔹 Verify round-trip
    assert_eq!(kubconfig, deserialized);
    println!("✓ Serialization and deserialization successful!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{Token, assert_de_tokens};

    #[test]
    fn test_ser_de() {
        let c = KubeConfig {
            port: 1181,
            healthz_port: 2291,
            max_pods: 15,
        };

        assert_de_tokens(
            &c,
            &[
                Token::Struct {
                    name: "KubeConfig",
                    len: 3,
                },
                Token::BorrowedStr("port"),
                Token::U16(1181),
                Token::BorrowedStr("healthz_port"),
                Token::U16(2291),
                Token::BorrowedStr("max_pods"),
                Token::U16(15),
                Token::StructEnd,
            ],
        )
    }
}
