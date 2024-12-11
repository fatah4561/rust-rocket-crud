use crate::exception::error::CustomError;
use crate::models::common_model::MapValueType;
use std::collections::BTreeMap;

pub fn insert_unsigned_validation(
    validations: &mut BTreeMap<String, MapValueType>,
    field_name: &str,
    value: Option<&str>,
) -> Result<(), CustomError> {
    if let Some(val) = value {
        let parsed_value = val
            .parse::<u64>()
            .map_err(|e| CustomError::internal_server_error(e.to_string()))?;
        validations.insert(field_name.to_string(), MapValueType::Unsigned(parsed_value));
    }
    Ok(())
}
