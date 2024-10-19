use std::fmt::Display;
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use crate::core::errors::auth_errors::AuthError;
use crate::core::permissions::permission_constants::equipment_permissions::EQUIP_READ_PERMISSION;
use crate::core::permissions::permission_constants::user_permissions::USER_READ_PERMISSION;
use crate::features::auth::key_creation_and_retrieval::claims::Claims;

#[derive(Debug)]
pub struct PermissionsManager {
    pub claims: Claims,
    pub admin_permissions: FeaturePermissions,
    pub user_permissions: FeaturePermissions,
    pub equipment_permissions: FeaturePermissions,
}

#[async_trait]
impl<S> FromRequestParts<S> for PermissionsManager
where S: Send + Sync {
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let claims_result = Claims::from_request_parts(parts, _state).await;
        let claims = match claims_result {
            Ok(claims) => claims,
            Err(_) => return Err(AuthError::InvalidToken),
        };
        // extract permission_constants from claims permissions_bitwise
        let  permissions_option = PermissionsManager::from_permissions_bitwise(&claims.permissions_bitwise);
        let mut permissions = match permissions_option {
            Some(permissions) => permissions,
            None => return Err(AuthError::InvalidToken),
        };
        permissions.claims = claims;
        Ok(permissions)
    }
}

impl Display for PermissionsManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_permissions_bitwise())
    }
}

pub const PERMISSION_WIDTH: usize = 4;
impl PermissionsManager {
    pub fn new() -> Self {
        PermissionsManager {
            claims: Claims::new(),
            admin_permissions: FeaturePermissions::new(),
            user_permissions: FeaturePermissions::new(),
            equipment_permissions: FeaturePermissions::new(),
        }
    }

    pub fn Default() -> Self {
        let mut default = PermissionsManager {
            claims: Claims::new(),
            admin_permissions: FeaturePermissions::new(),
            user_permissions: FeaturePermissions::new(),
            equipment_permissions: FeaturePermissions::new(),
        };

        // Set default permissions every user should have on sign up
        default.user_permissions.set_permission(USER_READ_PERMISSION);
        default.equipment_permissions.set_permission(EQUIP_READ_PERMISSION);

        default
    }

    pub fn get_default_permissions_bitwise() -> String {
        PermissionsManager::Default().to_permissions_bitwise()
    }

    pub fn from_permissions_bitwise(permissions_bitwise: &String) -> Option<Self> {
        let permissions: Vec<_> = permissions_bitwise.split("-").collect();

        if permissions.len() != 3 {
            return None;
        }

        Some(PermissionsManager {
            claims: Claims::new(),
            admin_permissions: FeaturePermissions::from_string(&permissions[0].to_string()).unwrap(),
            user_permissions: FeaturePermissions::from_string(&permissions[1].to_string()).unwrap(),
            equipment_permissions: FeaturePermissions::from_string(&permissions[2].to_string()).unwrap(),
        })
    }

    pub fn to_permissions_bitwise(&self) -> String {
        let mut permissions_bitwise = String::new();

        permissions_bitwise.push_str(format!("{:0width$b}", self.admin_permissions.bits, width = PERMISSION_WIDTH).as_str());
        permissions_bitwise.push_str("-");
        permissions_bitwise.push_str(format!("{:0width$b}", self.user_permissions.bits, width = PERMISSION_WIDTH).as_str());
        permissions_bitwise.push_str("-");
        permissions_bitwise.push_str(format!("{:0width$b}", self.equipment_permissions.bits, width = PERMISSION_WIDTH).as_str());

        permissions_bitwise
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FeaturePermissions {
    pub bits: u32,
}

impl FeaturePermissions {
    fn new() -> Self {
        FeaturePermissions { bits: 0 }
    }

    fn from_string(permissions_bitwise: &String) -> Result<Self, String> {
        if permissions_bitwise.len() != PERMISSION_WIDTH {
            return Err(format!("Invalid permission string format. Current binary length is {}.", PERMISSION_WIDTH));
        }

        if let Ok(bits) = u32::from_str_radix(permissions_bitwise, 2) {
            Ok(FeaturePermissions { bits })
        } else {
            Err(String::from("Invalid permission string format. Only binary strings allowed."))
        }
    }

    

    pub fn set_permission(&mut self, permission: u32) {
        self.bits |= permission;
    }

    pub fn remove_permission(&mut self, permission: u32) {
        self.bits &= !permission;
    }

    pub fn has_permission(&self, permission: u32) -> bool {
        (self.bits & permission) != 0
    }

    pub fn set_permission_group(&mut self, group: u32) {
        self.bits |= group;
    }

    pub fn remove_permission_group(&mut self, group: u32) {
        self.bits &= !group;
    }

    pub fn has_permission_group(&self, group: u32) -> bool {
        (self.bits & group) == group
    }

    pub fn any_permission(&self) -> bool { self.bits != 0 }
    
    pub fn clear_permissions(&mut self) {
        self.bits = 0;
    }
}