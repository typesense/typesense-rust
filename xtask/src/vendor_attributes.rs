use crate::preprocess_openapi::{OpenAPI, OpenAPIProperty};
use indexmap::IndexMap;
use serde_yaml::Value;

/// Main helper struct that holds a mutable borrow of the OpenAPI root mapping.
pub struct VendorAttributes<'a> {
    pub doc: &'a mut OpenAPI,
}

impl<'a> VendorAttributes<'a> {
    pub fn new(doc: &'a mut OpenAPI) -> Self {
        Self { doc }
    }

    pub fn schema_generic_parameter<const N: usize>(
        &mut self,
        items: [(&str, &str); N],
    ) -> Result<&mut Self, String> {
        for (schema_name, generic) in items {
            let map = self
                .doc
                .components
                .schemas
                .get_mut(schema_name)
                .ok_or_else(|| format!("schema not found: {schema_name}"))?;

            map.extra.insert(
                "x-rust-generic-parameter".to_owned(),
                Value::String(generic.into()),
            );
        }
        Ok(self)
    }

    pub fn schema_builder<const N: usize>(
        &mut self,
        schemas: [&str; N],
    ) -> Result<&mut Self, String> {
        for schema_name in schemas {
            let map = self
                .doc
                .components
                .schemas
                .get_mut(schema_name)
                .ok_or_else(|| format!("schema not found: {schema_name}"))?;

            map.extra
                .insert("x-rust-builder".to_owned(), Value::Bool(true));
        }
        Ok(self)
    }

    pub fn schema_field_type_overrides<const N: usize>(
        &mut self,
        schema: &str,
        overrides: [(&str, &str); N],
    ) -> Result<&mut Self, String> {
        for (field, rust_type) in overrides {
            let props_map = self
                .doc
                .components
                .schemas
                .get_mut(schema)
                .ok_or_else(|| format!("schema not found: {schema}"))?
                .properties
                .as_mut()
                .ok_or_else(|| format!("No properties in schema: {schema}"))?;
            match props_map.get_mut(field) {
                Some(existing_val) => {
                    existing_val
                        .extra
                        .insert("x-rust-type".to_owned(), Value::String(rust_type.into()));
                }
                None => {
                    let new_field_map = OpenAPIProperty {
                        extra: IndexMap::from([(
                            "x-rust-type".to_owned(),
                            Value::String(rust_type.into()),
                        )]),
                        ..Default::default()
                    };
                    props_map.insert(field.to_owned(), new_field_map);
                }
            }
        }
        Ok(self)
    }

    pub fn operation<'b>(&'b mut self, path: &'b str, method: &'b str) -> OperationContext<'a, 'b> {
        OperationContext {
            vendor: self,
            path,
            method,
        }
    }
}

pub struct OperationContext<'a, 'b> {
    vendor: &'b mut VendorAttributes<'a>,
    path: &'b str,
    method: &'b str,
}

impl<'a, 'b> OperationContext<'a, 'b> {
    fn try_set(&mut self, attr: &str, val: Value) -> Result<&mut Self, String> {
        let method = self
            .vendor
            .doc
            .paths
            .get_mut(self.path)
            .ok_or_else(|| format!("operation path not found: {}", self.path))?
            .get_mut(self.method)
            .ok_or_else(|| format!("operation method not found: {}.{}", self.path, self.method))?;
        method.extra.insert(attr.to_owned(), val);
        Ok(self)
    }

    fn try_set_request_body(&mut self, attr: &str, val: Value) -> Result<&mut Self, String> {
        let req = &mut self
            .vendor
            .doc
            .paths
            .get_mut(self.path)
            .ok_or_else(|| format!("operation path not found: {}", self.path))?
            .get_mut(self.method)
            .ok_or_else(|| format!("operation method not found: {}.{}", self.path, self.method))?
            .request_body;
        let req = match req {
            Some(v) => v,
            None => {
                *req = Some(Default::default());
                req.as_mut().unwrap()
            }
        };
        req.extra.insert(attr.to_owned(), val);
        Ok(self)
    }

    pub fn generic_parameter(&mut self, generic: &str) -> Result<&mut Self, String> {
        self.try_set("x-rust-generic-parameter", Value::String(generic.into()))
    }

    pub fn params_generic_parameter(&mut self, generic: &str) -> Result<&mut Self, String> {
        self.try_set_request_body(
            "x-rust-params-generic-parameter",
            Value::String(generic.into()),
        )
    }

    pub fn return_type(&mut self, typ: &str) -> Result<&mut Self, String> {
        self.try_set("x-rust-return-type", Value::String(typ.into()))
    }

    pub fn request_type(&mut self, typ: &str) -> Result<&mut Self, String> {
        self.try_set_request_body("x-rust-type", Value::String(typ.into()))
    }

    pub fn body_is_raw_text(&mut self) -> Result<&mut Self, String> {
        self.try_set("x-rust-body-is-raw-text", Value::Bool(true))
    }

    /// Indicate that the response supports plain text besides JSON
    pub fn supports_plain_text(&mut self) -> Result<&mut Self, String> {
        self.try_set("x-supports-plain-text", Value::Bool(true))
    }
}
