use serde_yaml::{Mapping, Value};

/// Where to apply a vendor (x-*) attribute.
pub enum VendorLocation<'a> {
    Schema(&'a str),
    SchemaField { schema: &'a str, field: &'a str },
    Operation { path: &'a str, method: &'a str },
}

/// Main helper struct that holds a mutable borrow of the OpenAPI root mapping.
pub struct VendorAttributes<'a> {
    doc: &'a mut Mapping,
}

impl<'a> VendorAttributes<'a> {
    pub fn new(doc: &'a mut Mapping) -> Self {
        Self { doc }
    }

    // internal helpers

    fn traverse_value_mut(&mut self, keys: &[&str]) -> Option<&mut Value> {
        if keys.is_empty() {
            return None;
        }
        let mut cur: Option<&mut Value> = self.doc.get_mut(keys[0]);
        for k in &keys[1..] {
            cur = cur.and_then(|v| v.get_mut(k));
        }
        cur
    }

    fn get_map_mut(&mut self, keys: &[&str]) -> Result<&mut Mapping, String> {
        self.traverse_value_mut(keys)
            .and_then(|v| v.as_mapping_mut())
            .ok_or_else(|| format!("expected mapping at path: {}", keys.join(".")))
    }

    #[inline]
    fn insert_into_map(map: &mut Mapping, attr: &str, val: Value) {
        map.insert(Value::String(attr.to_string()), val);
    }

    fn set_attr(
        &mut self,
        location: VendorLocation<'_>,
        attr: &str,
        val: Value,
    ) -> Result<&mut Self, String> {
        match location {
            VendorLocation::Schema(schema_name) => {
                let map = self.get_map_mut(&["components", "schemas", schema_name])?;
                Self::insert_into_map(map, attr, val);
                Ok(self)
            }
            VendorLocation::SchemaField { schema, field } => {
                let props_map = self
                    .get_map_mut(&["components", "schemas", schema, "properties"])
                    .map_err(|_| format!("schema '{}' has no properties mapping", schema))?;

                let prop_key = Value::String(field.to_string());
                match props_map.get_mut(&prop_key) {
                    Some(existing_val) => {
                        if let Some(field_map) = existing_val.as_mapping_mut() {
                            Self::insert_into_map(field_map, attr, val);
                            Ok(self)
                        } else {
                            Err(format!(
                                "property '{}' in schema '{}' exists but is not a mapping; cannot set '{}'",
                                field, schema, attr
                            ))
                        }
                    }
                    None => {
                        let mut new_field_map = Mapping::new();
                        new_field_map.insert(Value::String(attr.to_string()), val);
                        props_map.insert(prop_key, Value::Mapping(new_field_map));
                        Ok(self)
                    }
                }
            }
            VendorLocation::Operation { path, method } => {
                let op_map = self
                    .get_map_mut(&["paths", path, method])
                    .map_err(|_| format!("operation not found: {} {}", method, path))?;
                Self::insert_into_map(op_map, attr, val);
                Ok(self)
            }
        }
    }

    pub fn schema_generic_parameter<const N: usize>(
        &mut self,
        items: [(&str, &str); N],
    ) -> Result<&mut Self, String> {
        for (schema, generic) in items {
            self.set_attr(
                VendorLocation::Schema(schema),
                "x-rust-generic-parameter",
                Value::String(generic.into()),
            )?;
        }
        Ok(self)
    }

    pub fn schema_builder<const N: usize>(
        &mut self,
        schemas: [&str; N],
    ) -> Result<&mut Self, String> {
        for schema in schemas {
            self.set_attr(
                VendorLocation::Schema(schema),
                "x-rust-builder",
                Value::Bool(true),
            )?;
        }
        Ok(self)
    }

    pub fn schema_field_type_overrides<const N: usize>(
        &mut self,
        schema: &str,
        overrides: [(&str, &str); N],
    ) -> Result<&mut Self, String> {
        for (field, rust_type) in overrides {
            self.set_attr(
                VendorLocation::SchemaField { schema, field },
                "x-rust-type",
                Value::String(rust_type.into()),
            )?;
        }
        Ok(self)
    }

    pub fn operation<'b>(&'b mut self, path: &'b str, method: &'b str) -> OperationContext<'a, 'b> {
        OperationContext {
            vendor: self,
            path,
            method,
            error: None,
        }
    }
}

pub struct OperationContext<'a, 'b> {
    vendor: &'b mut VendorAttributes<'a>,
    path: &'b str,
    method: &'b str,
    error: Option<String>,
}

impl<'a, 'b> OperationContext<'a, 'b> {
    fn try_set(&mut self, attr: &str, val: Value) {
        if self.error.is_some() {
            return;
        }
        if let Err(e) = self.vendor.set_attr(
            VendorLocation::Operation {
                path: self.path,
                method: self.method,
            },
            attr,
            val,
        ) {
            self.error = Some(e);
        }
    }

    pub fn generic_parameter(mut self, generic: &str) -> Self {
        self.try_set("x-rust-generic-parameter", Value::String(generic.into()));
        self
    }

    pub fn return_type(mut self, rust_type: &str) -> Self {
        self.try_set("x-rust-return-type", Value::String(rust_type.into()));
        self
    }

    pub fn body_is_raw_text(mut self) -> Self {
        self.try_set("x-rust-body-is-raw-text", Value::Bool(true));
        self
    }

    pub fn supports_plain_text(mut self) -> Self {
        self.try_set("x-supports-plain-text", Value::Bool(true));
        self
    }

    /// Return to VendorAttributes if no errors, or propagate the first error
    pub fn done(self) -> Result<&'b mut VendorAttributes<'a>, String> {
        match self.error {
            Some(err) => Err(err),
            None => Ok(self.vendor),
        }
    }
}
