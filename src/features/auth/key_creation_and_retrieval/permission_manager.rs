use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use crate::core::errors::auth_errors::AuthError;
use crate::features::auth::key_creation_and_retrieval::claims;

#[derive(Debug)]
struct PermissionsManager {
    admin_permissions: FeaturePermissions,
    user_permissions: FeaturePermissions,
    equipment_permissions: FeaturePermissions,
}

#[async_trait]
impl<S> FromRequestParts<S> for PermissionsManager
where S: Send + Sync {
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let claims_result = claims::Claims::from_request_parts(parts, _state).await;
        let claims = match claims_result {
            Ok(claims) => claims,
            Err(_) => return Err(AuthError::InvalidToken),
        };
        // extract permissions from claims permissions_bitwise
        let permissions = PermissionsManager::from_permissions_bitwise(&claims.permissions_bitwise);
        Ok(permissions)
    }
}

impl PermissionsManager {
    fn new() -> Self {
        PermissionsManager {
            admin_permissions: FeaturePermissions::new(),
            user_permissions: FeaturePermissions::new(),
            equipment_permissions: FeaturePermissions::new(),
        }
    }
    fn from_permissions_bitwise(permissions_bitwise: &String) -> Self {
        let permissions: Vec<_> = permissions_bitwise.split("-").collect();
        
        Some(PermissionsManager {
            admin_permissions: FeaturePermissions::from_string(&permissions[0].to_string()).unwrap(),
            user_permissions: FeaturePermissions::from_string(&permissions[1].to_string()).unwrap(),
            equipment_permissions: FeaturePermissions::from_string(&permissions[2].to_string()).unwrap(),
        }).unwrap_or_else(|| PermissionsManager::new())
    }
}

#[derive(Debug, Clone, Copy)]
struct FeaturePermissions {
    bits: u32,
}

impl FeaturePermissions {
    fn new() -> Self {
        FeaturePermissions { bits: 0 }
    }
    
    fn from_string(permissions_bitwise: &String) -> Result<Self, &'static str> {
        if let Ok(bits) = u32::from_str_radix(permissions_bitwise, 2) {
            Ok(FeaturePermissions { bits })
        } else {
            Err("Invalid permission string format. Only binary strings allowed.")
        }
    }


    fn set_permission(&mut self, permission: u32) {
        self.bits |= permission;
    }

    fn remove_permission(&mut self, permission: u32) {
        self.bits &= !permission;
    }

    fn has_permission(&self, permission: u32) -> bool {
        (self.bits & permission) != 0
    }

    fn set_permission_group(&mut self, group: u32) {
        self.bits |= group;
    }

    fn remove_permission_group(&mut self, group: u32) {
        self.bits &= !group;
    }

    fn has_permission_group(&self, group: u32) -> bool {
        (self.bits & group) == group
    }

    fn clear_permissions(&mut self) {
        self.bits = 0;
    }
}